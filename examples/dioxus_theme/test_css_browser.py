"""CSS and scope contracts. Serve `dx serve --example css_scope --web --port 8081`."""
import os

from playwright.sync_api import expect, sync_playwright


with sync_playwright() as p:
    browser = p.chromium.launch(
        headless=True, executable_path=os.environ.get('PLAYWRIGHT_CHROMIUM_EXECUTABLE')
    )
    page = browser.new_page()
    errors = []
    page.on('pageerror', lambda error: errors.append(str(error)))
    page.goto(os.environ.get('FERRISWATCH_CSS_TEST_URL', 'http://127.0.0.1:8081'))
    page.wait_for_selector('#counter')

    def background(selector):
        return page.locator(selector).evaluate('(el) => getComputedStyle(el).backgroundColor')

    def variable(selector, name='background'):
        return page.locator(selector).evaluate(
            '(el, name) => getComputedStyle(el).getPropertyValue("--fs-" + name).trim()', name
        )

    # A provider alone never loads class CSS.
    assert page.locator('link[rel=stylesheet]').count() == 0
    assert background('#outside') == 'rgba(0, 0, 0, 0)'
    dark = variable('html')
    assert dark != 'rgb(1, 2, 3)'
    light = variable('#nested')
    assert light != dark
    page.locator('#styles').click()
    expect(page.locator('link[rel=stylesheet]')).to_have_count(1)
    page.wait_for_function('getComputedStyle(document.querySelector("#outside")).backgroundColor !== "rgba(0, 0, 0, 0)"')
    sheet = page.locator('link[rel=stylesheet]').get_attribute('href')
    assert background('#outside') == background('#inside')
    assert background('#nested') != background('#inside')

    page.locator('#counter').click()
    page.locator('#mode').click()
    assert variable('html') == light
    assert background('#inside') == background('#nested')
    page.locator('#mode').click()
    assert variable('html') == dark
    page.locator('#scope').click()
    assert variable('html') == 'rgb(1, 2, 3)'
    assert variable('#inside') == dark
    expect(page.locator('#counter')).to_have_text('1')
    expect(page.locator('style[data-fs-root]')).to_have_count(0)
    page.locator('#scope').click()
    assert variable('html') == dark
    expect(page.locator('#counter')).to_have_text('1')
    page.locator('#mount').click()
    assert variable('html') == 'rgb(1, 2, 3)'
    assert page.locator('html').evaluate('(el) => getComputedStyle(el).colorScheme') == 'light'
    expect(page.locator('style[data-fs-root]')).to_have_count(0)
    page.locator('#mount').click()
    expect(page.locator('#counter')).to_have_text('0')
    assert variable('html') == dark
    expect(page.locator('link[rel=stylesheet]')).to_have_count(1)
    assert page.locator('link[rel=stylesheet]').get_attribute('href') == sheet

    # Use a DOM fixture without application CSS to test the actual loaded asset.
    page.evaluate('''() => {
        const host = document.createElement('div');
        host.id = 'contracts';
        host.innerHTML = `
            <div id="card" class="fs-card fs-bg-raised fs-text-muted fs-border"></div>
            <div id="raised" class="fs-bg-raised"></div>
            <div id="muted" class="fs-text-muted"></div>
            <div id="border" class="fs-border"></div>
            <div id="overridden" class="fs-card"></div>
            <button id="primary" class="fs-primary fs-focus">Primary</button>
            <button id="secondary" class="fs-secondary fs-focus">Secondary</button>
            <fieldset disabled><button id="inherited-disabled" class="fs-primary">Disabled fieldset</button></fieldset>
            <div id="probe"></div>`;
        document.body.append(host);
    }''')
    assert background('#card') == background('#raised')
    for property_name, expected in [('color', '#muted'), ('borderTopColor', '#border')]:
        assert page.locator('#card').evaluate('(el, p) => getComputedStyle(el)[p]', property_name) == page.locator(expected).evaluate('(el, p) => getComputedStyle(el)[p]', property_name)
    # Application CSS works even when it precedes the library stylesheet.
    page.evaluate('''() => {
        const style = document.createElement('style');
        style.textContent = '#overridden { background-color: rgb(9, 8, 7); }';
        document.head.prepend(style);
    }''')
    assert background('#overridden') == 'rgb(9, 8, 7)'

    def expected_color(role):
        return page.locator('#probe').evaluate('''(el, role) => {
            el.style.backgroundColor = 'var(--fs-' + role + ')';
            return getComputedStyle(el).backgroundColor;
        }''', role)

    for role in ['primary', 'secondary']:
        button = page.locator('#' + role)
        page.mouse.move(0, 0)
        assert background('#' + role) == expected_color(role)
        button.hover()
        assert background('#' + role) == expected_color(role + '-hover')
        page.mouse.down()
        assert background('#' + role) == expected_color(role + '-pressed')
        page.mouse.up()
        assert background('#' + role) == expected_color(role + '-hover')
        for attribute in ['disabled', 'aria-disabled']:
            button.evaluate('(el, attr) => el.setAttribute(attr, "true")', attribute)
            assert background('#' + role) == expected_color(role)
            page.mouse.down()
            assert background('#' + role) == expected_color(role)
            page.mouse.up()
            button.evaluate('(el, attr) => el.removeAttribute(attr)', attribute)
    page.locator('#inherited-disabled').hover(force=True)
    assert background('#inherited-disabled') == expected_color('primary')
    # Tab onto an action; focus belongs to the explicit helper.
    page.locator('#primary').focus()
    page.keyboard.press('Tab')
    expect(page.locator('#secondary')).to_be_focused()
    assert page.locator('#secondary').evaluate('''(el) => {
        const s = getComputedStyle(el);
        return el.matches(':focus-visible') && s.outlineWidth === '2px'
            && s.outlineStyle === 'solid' && s.outlineOffset === '2px';
    }''')
    assert not errors, errors
    browser.close()
    print('CSS cascade, states, optional loading, and scope lifecycle checks passed')
