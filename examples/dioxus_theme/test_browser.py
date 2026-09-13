"""Browser regression checks. Run against the served Dioxus example."""
import os

from playwright.sync_api import sync_playwright, expect
with sync_playwright() as p:
    browser = p.chromium.launch(headless=True, executable_path=os.environ.get('PLAYWRIGHT_CHROMIUM_EXECUTABLE'))
    context = browser.new_context(permissions=['clipboard-read', 'clipboard-write'])
    page = context.new_page()
    errors = []
    page.on('pageerror', lambda error: errors.append(str(error)))
    page.goto(os.environ.get('FERRISWATCH_EXAMPLE_URL', 'http://127.0.0.1:8080'))
    page.wait_for_load_state('networkidle')
    mode = page.get_by_role('switch', name='Dark mode')
    def set_mode(value):
        expected = 'true' if value == 'dark' else 'false'
        if mode.get_attribute('aria-checked') != expected:
            mode.click()
        expect(mode).to_have_attribute('aria-checked', expected)
    expect(mode).to_have_attribute('aria-checked', 'true')
    mode.focus()
    page.keyboard.press('Space')
    expect(mode).to_have_attribute('aria-checked', 'false')
    page.keyboard.press('Space')
    expect(mode).to_have_attribute('aria-checked', 'true')
    theme = page.get_by_label('Theme', exact=True)
    accent = page.get_by_label('Accent', exact=True)
    expect(theme).to_have_value('catppuccin/mocha')
    expect(accent).to_have_value('mauve')
    assert theme.locator('option').count() >= 5
    page.get_by_role('button', name='Color reference', exact=True).click()
    expect(page.locator('.project')).to_be_hidden()
    expect(page.locator('.reference')).to_be_visible()
    page.get_by_role('button', name='Interface', exact=True).click()
    expect(page.locator('.project')).to_be_visible()
    expect(page.locator('.reference')).to_have_count(0)
    def assert_theme_background():
        assert page.locator('.fw-theme').evaluate('''(el) => {
            const expected = document.createElement('div');
            expected.style.backgroundColor = 'var(--fw-background)';
            el.append(expected);
            const matches = getComputedStyle(el).backgroundColor === getComputedStyle(expected).backgroundColor;
            expected.remove();
            return matches;
        }'''), 'Theme background must match its palette after every update'

    assert_theme_background()
    page.get_by_role('textbox', name='Project name').fill('preserved')
    original = page.locator('button.primary').evaluate('(el) => getComputedStyle(el).backgroundColor')
    accent.select_option('blue')
    page.wait_for_function('(original) => getComputedStyle(document.querySelector("button.primary")).backgroundColor !== original', arg=original)
    assert_theme_background()
    dark_options = theme.locator('option').evaluate_all('(options) => options.map(o => o.value)')
    assert 'catppuccin/latte' not in dark_options
    assert 'catppuccin/mocha' in dark_options
    set_mode('light')
    expect(theme).to_have_value('catppuccin/latte')
    expect(accent).to_have_value('blue')
    light_options = theme.locator('option').evaluate_all('(options) => options.map(o => o.value)')
    assert 'catppuccin/mocha' not in light_options
    assert 'nord/main' not in light_options
    accent.select_option('green')
    theme.select_option('rose_pine/dawn')
    expect(theme).to_have_value('rose_pine/dawn')
    expect(accent).to_have_value('')
    accent.select_option('rose')
    set_mode('dark')
    expect(theme).to_have_value('catppuccin/mocha')
    expect(accent).to_have_value('blue')
    set_mode('light')
    expect(theme).to_have_value('rose_pine/dawn')
    expect(accent).to_have_value('rose')
    page.wait_for_function('getComputedStyle(document.querySelector(".fw-theme")).colorScheme === "light"')
    assert_theme_background()
    set_mode('dark')
    theme.select_option('nord/main')
    expect(accent).to_have_value('')
    assert_theme_background()
    page.get_by_role('button', name='Reset theme').click()
    expect(theme).to_have_value('catppuccin/mocha')
    expect(accent).to_have_value('mauve')
    expect(mode).to_have_attribute('aria-checked', 'true')
    set_mode('light')
    expect(theme).to_have_value('catppuccin/latte')
    expect(accent).to_have_value('blue')
    set_mode('dark')
    assert_theme_background()
    expect(page.get_by_role('textbox', name='Project name')).to_have_value('preserved')
    page.get_by_role('button', name='Save changes').click()
    expect(page.get_by_role('heading', name='preserved', exact=True)).to_be_visible()
    expect(page.get_by_role('status')).to_have_text('Saved for this session.')
    page.get_by_role('textbox', name='New task').fill('Check mobile layout')
    page.get_by_role('button', name='Add task', exact=True).click()
    new_task = page.get_by_role('checkbox', name='Check mobile layout')
    new_task.check()
    expect(page.get_by_role('progressbar')).to_have_attribute('value', '2')
    expect(page.get_by_role('progressbar')).to_have_attribute('max', '4')
    expect(page.get_by_role('button', name='Add task', exact=True)).to_be_disabled()
    page.get_by_role('button', name='Color reference', exact=True).click()
    expect(page.locator('.swatch')).to_have_count(32)
    page.get_by_role('button', name='Copy --fw-primary', exact=True).click()
    expect(page.locator('.copy-status')).to_have_text('Copied --fw-primary')
    assert page.evaluate('navigator.clipboard.readText()') == '--fw-primary'
    page.get_by_role('button', name='Copy --fw-alt-background', exact=True).focus()
    page.keyboard.press('Enter')
    expect(page.locator('.copy-status')).to_have_text('Copied --fw-alt-background')
    assert page.evaluate('navigator.clipboard.readText()') == '--fw-alt-background'
    page.evaluate("() => { window.originalWriteText = navigator.clipboard.writeText; navigator.clipboard.writeText = () => Promise.reject(new Error('Denied')); }")
    page.get_by_role('button', name='Copy --fw-error', exact=True).click()
    expect(page.locator('.copy-status')).to_contain_text("Couldn't copy")
    page.evaluate('navigator.clipboard.writeText = window.originalWriteText; delete window.originalWriteText')
    set_mode('light')
    assert_theme_background()
    page.get_by_role('button', name='Interface', exact=True).click()
    expect(new_task).to_be_checked()
    expect(page.get_by_role('textbox', name='Project name')).to_have_value('preserved')
    page.get_by_role('textbox', name='Project name').fill('   ')
    expect(page.get_by_role('button', name='Save changes')).to_be_disabled()
    for width in [1440, 768, 390, 320]:
        page.set_viewport_size({'width': width, 'height': 900})
        assert page.evaluate('document.documentElement.scrollWidth <= innerWidth'), f'Overflow at {width}px'
        if width <= 640:
            mode_box = mode.bounding_box()
            theme_box = theme.bounding_box()
            assert theme_box['y'] >= mode_box['y'] + mode_box['height']
            accent_box = accent.bounding_box()
            assert accent_box['y'] >= theme_box['y'] + theme_box['height'], 'Mobile selectors must stack'
            assert abs(theme_box['width'] - accent_box['width']) < 1
        page.get_by_role('button', name='Color reference', exact=True).click()
        expect(page.locator('.swatch')).to_have_count(32)
        assert page.evaluate('document.documentElement.scrollWidth <= innerWidth'), f'Reference overflow at {width}px'
        page.get_by_role('button', name='Interface', exact=True).click()
    page.set_viewport_size({'width': 390, 'height': 700})
    scrollbar = page.get_by_role('scrollbar', name='Page scroll')
    expect(scrollbar).to_be_visible()
    assert page.evaluate('document.documentElement.clientWidth === innerWidth'), 'Scrollbar must not reserve a gutter'
    scrollbar.focus()
    page.keyboard.press('End')
    page.wait_for_function('scrollY > 0 && Math.abs(scrollY + innerHeight - document.scrollingElement.scrollHeight) < 2')
    page.keyboard.press('Home')
    page.wait_for_function('scrollY === 0')
    thumb = page.locator('overlay-scrollbar > span').bounding_box()
    page.mouse.move(thumb['x'] + thumb['width'] / 2, thumb['y'] + 8)
    page.mouse.down()
    page.mouse.move(thumb['x'] + thumb['width'] / 2, 400, steps=8)
    page.mouse.up()
    page.wait_for_function('scrollY > 0')
    page.evaluate('scrollTo(0, 0)')
    page.mouse.move(150, 300)
    page.mouse.wheel(0, 200)
    page.wait_for_function('scrollY > 0')
    assert not errors, errors
    print('PASS: independent mode selections and filtering, theme switching, backgrounds, reset, saved edits, tasks, 32 colors, state retention, responsive layouts, no page errors')
    browser.close()
