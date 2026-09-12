#!/usr/bin/env python3
"""Exercise the generated inspector with Playwright and save every palette.

Serve target/ on localhost:8765 first. Requires the Python playwright package
and Chromium at /usr/bin/chromium. Screenshots stay under target/.
"""

from pathlib import Path
from playwright.sync_api import sync_playwright

out = Path(__file__).resolve().parents[1] / "target/palette-screenshots"
out.mkdir(exist_ok=True)
with sync_playwright() as p:
    browser = p.chromium.launch(
        headless=True, executable_path="/usr/bin/chromium", args=["--no-sandbox"]
    )
    page = browser.new_page(
        viewport={"width": 1420, "height": 1100}, device_scale_factor=1
    )
    errors = []
    page.on("pageerror", lambda e: errors.append(str(e)))
    page.goto("http://127.0.0.1:8765/palette-inspector.html")
    page.wait_for_load_state("networkidle")
    assert page.locator("article").count() == 64
    result = page.evaluate(
        """() => { let selections=0; const failures=[]; for(const card of document.querySelectorAll('article')) {const select=card.querySelector('select');for(const option of select.options){select.value=option.value;select.dispatchEvent(new Event('change'));selections++; const primary=getComputedStyle(card.querySelector('.primary')).backgroundColor; if(!CSS.supports('color',primary)|| primary==='rgba(0, 0, 0, 0)') failures.push(card.dataset.id+':'+option.value); }select.value='';select.dispatchEvent(new Event('change'));}return {selections,failures};}"""
    )
    assert not result["failures"], result
    # Every palette is rendered and saved, in eight reviewable sheets of eight samples.
    for batch in range(8):
        page.evaluate(
            '(batch)=>{[...document.querySelectorAll("article")].forEach((el,i)=>el.hidden=Math.floor(i/8)!==batch)}',
            batch,
        )
        page.screenshot(path=str(out / f"sheet-{batch + 1}.png"), full_page=True)
    # Individual samples give readable evidence at native size.
    for id in [
        "catppuccin/mocha",
        "kanagawa/lotus",
        "material/lighter",
        "jetbrains/dark",
        "panda/main",
        "vscode/light_plus",
    ]:
        page.locator("#search").fill(id)
        assert page.locator("article:visible").count() == 1
        page.screenshot(path=str(out / (id.replace("/", "-") + ".png")), full_page=True)
    page.locator("#search").fill("does-not-exist")
    assert page.locator("article:visible").count() == 0
    page.locator("#search").fill("")
    assert page.locator("article:visible").count() == 64
    page.set_viewport_size({"width": 390, "height": 844})
    page.locator("#search").fill("catppuccin/mocha")
    assert page.evaluate("document.documentElement.scrollWidth<=window.innerWidth")
    page.screenshot(path=str(out / "mobile.png"), full_page=True)
    assert not errors, errors
    print(result, "screenshots", len(list(out.glob("*.png"))), "browser errors", errors)
    browser.close()
