"""Browser contract for the opt-in Dioxus Components theme adapter."""
import os

from playwright.sync_api import sync_playwright


with sync_playwright() as p:
    browser = p.chromium.launch(
        headless=True, executable_path=os.environ.get("PLAYWRIGHT_CHROMIUM_EXECUTABLE")
    )
    page = browser.new_page(color_scheme="light")
    page.goto(os.environ.get("FERRISWATCH_DX_THEME_TEST_URL", "http://127.0.0.1:8081"))
    page.wait_for_selector("#root-primary", state="attached")

    # The upstream sheet follows provider rules. Contradictory document state must
    # still be observable: data-theme says dark while the simulated OS says light.
    page.evaluate("document.documentElement.setAttribute('data-theme', 'dark')")

    def css(selector, prop):
        return page.locator(selector).evaluate("(el, p) => getComputedStyle(el)[p]", prop)

    def custom(selector, name):
        return page.locator(selector).evaluate(
            "(el, n) => getComputedStyle(el).getPropertyValue(n).trim()", name
        )

    def resolved(selector, name, prop="color"):
        return page.locator(selector).evaluate(
            """(el, args) => {
                const old = el.style[args.prop];
                el.style[args.prop] = `var(${args.name})`;
                const value = getComputedStyle(el)[args.prop];
                el.style[args.prop] = old;
                return value;
            }""",
            {"name": name, "prop": prop},
        )

    upstream_dark = css("#outside-probe", "backgroundColor")
    assert upstream_dark == "rgb(0, 0, 0)"
    assert custom("#root-probe", "--primary-color") != ""

    # Opt-in is off initially, then captured on a keyed provider remount.
    page.locator("#adapter").click()
    assert css("#root-primary", "backgroundColor") == resolved("#root-primary", "--fs-background", "backgroundColor")
    assert css("#root-secondary", "color") == resolved("#root-secondary", "--fs-text")
    assert css("#root-focused", "borderTopColor") == resolved("#root-focused", "--fs-focus")
    for role in ["success", "warning", "info"]:
        assert css(f"#root-{role}", "color") == resolved(f"#root-{role}", f"--fs-{role}")

    # Render the switch fallbacks: `initial` is guaranteed-invalid, whereas
    # the other branch contains whitespace. CSSOM strings cannot distinguish them.
    assert css("#root-dark-flag", "backgroundColor") == "rgb(1, 2, 3)"
    assert css("#root-light-flag", "backgroundColor") == "rgba(0, 0, 0, 0)"
    assert css("#nested-light-flag", "backgroundColor") == "rgb(4, 5, 6)"
    assert css("#nested-primary", "backgroundColor") != css("#root-primary", "backgroundColor")
    page.locator("#mode").click()
    assert css("#root-dark-flag", "backgroundColor") == "rgba(0, 0, 0, 0)"
    assert css("#root-light-flag", "backgroundColor") == "rgb(4, 5, 6)"
    assert css("#root-primary", "backgroundColor") == resolved("#root-primary", "--fs-background", "backgroundColor")
    assert css("#nested-primary", "backgroundColor") == css("#root-primary", "backgroundColor")
    page.locator("#mode").click()
    assert css("#nested-primary", "backgroundColor") != css("#root-primary", "backgroundColor")

    # Root/scoped switching changes where the override applies while preserving
    # descendants. Root mode also covers the outside document probe.
    page.locator("#scope").click()
    assert css("#outside-probe", "backgroundColor") == upstream_dark
    assert css("#root-primary", "backgroundColor") == resolved("#root-primary", "--fs-background", "backgroundColor")
    page.locator("#scope").click()
    assert css("#outside-probe", "backgroundColor") == resolved("#outside-probe", "--fs-background", "backgroundColor")

    # Removing and remounting restores upstream root values and reinstates the
    # adapter with a fresh provider state.
    page.locator("#mount").click()
    assert page.locator("#root-primary").count() == 0
    assert css("#outside-probe", "backgroundColor") == upstream_dark
    page.locator("#mount").click()
    assert css("#root-primary", "backgroundColor") == resolved("#root-primary", "--fs-background", "backgroundColor")
    assert page.locator("#nested-probe").count() == 1
    # Opting out removes the root overrides, including the system-switch rule.
    page.locator("#adapter").click()
    assert css("#outside-probe", "backgroundColor") == upstream_dark
    assert css("#root-primary", "backgroundColor") == upstream_dark
    assert css("#nested-primary", "backgroundColor") == resolved("#nested-primary", "--fs-background", "backgroundColor")
    browser.close()
    print("Dioxus Components adapter behavior passed")
