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
    def select_option(combo, value, label):
        """Choose a connected combobox option by stable ID and assert its label."""
        combo.click()
        option = page.locator(f'[role="option"][data-value="{value}"]')
        expect(option).to_be_visible()
        option.click()
        expect(combo).to_have_value(label)

    def option_values(list_label):
        return page.get_by_role('listbox', name=list_label).get_by_role('option').evaluate_all(
            '(options) => options.map(o => o.dataset.value)'
        )

    expect(theme).to_have_value('Catppuccin Mocha')
    expect(accent).to_have_value('Mauve')
    theme.click()
    themes = page.get_by_role('listbox', name='Themes')
    expect(themes).to_be_visible()
    assert len(option_values('Themes')) >= 5
    themes_box = themes.bounding_box()
    page.mouse.move(themes_box['x'] + themes_box['width'] / 2, themes_box['y'] + themes_box['height'] / 2)
    page.mouse.wheel(0, 260)
    page.wait_for_function("""(label) => {
        const el = [...document.querySelectorAll('[role="listbox"]')].find((node) => node.getAttribute('aria-label') === label);
        return el && el.scrollTop > 0;
    }""", arg='Themes')
    # Search has a real empty state, and Escape restores the selected label.
    theme.fill('zzzz-no-such-theme')
    expect(page.get_by_text('No themes found', exact=True)).to_be_visible()
    theme.press('Escape')
    expect(themes).to_be_hidden()
    expect(theme).to_have_value('Catppuccin Mocha')
    # Opening one connected combobox closes the other popup.
    theme.click()
    expect(themes).to_be_visible()
    accent.click()
    expect(themes).to_be_hidden()
    accents = page.get_by_role('listbox', name='Accents')
    expect(accents).to_be_visible()
    accents_box = accents.bounding_box()
    page.mouse.move(accents_box['x'] + accents_box['width'] / 2, accents_box['y'] + accents_box['height'] / 2)
    page.mouse.wheel(0, 260)
    page.wait_for_function("""(label) => {
        const el = [...document.querySelectorAll('[role="listbox"]')].find((node) => node.getAttribute('aria-label') === label);
        return el && el.scrollTop > 0;
    }""", arg='Accents')
    accent.press('Escape')
    # Both connected controls support searching and selecting with the keyboard.
    theme.click()
    theme.fill('Mocha')
    theme.press('ArrowDown')
    theme.press('Enter')
    expect(theme).to_have_value('Catppuccin Mocha')
    accent.click()
    accent.fill('Blue')
    accent.press('ArrowDown')
    accent.press('Enter')
    expect(accent).to_have_value('Blue')
    select_option(accent, 'mauve', 'Mauve')
    page.get_by_role('button', name='Color reference', exact=True).click()
    expect(page.locator('.project')).to_be_hidden()
    expect(page.locator('.reference')).to_be_visible()
    page.get_by_role('button', name='Interface', exact=True).click()
    expect(page.locator('.project')).to_be_visible()
    expect(page.locator('.reference')).to_have_count(0)
    def assert_theme_background():
        assert page.locator('.fs-theme').evaluate('''(el) => {
            const expected = document.createElement('div');
            expected.style.backgroundColor = 'var(--fs-background)';
            el.append(expected);
            const matches = getComputedStyle(el).backgroundColor === getComputedStyle(expected).backgroundColor;
            expected.remove();
            return matches;
        }'''), 'Theme background must match its palette after every update'

    assert_theme_background()
    assert page.locator('.project').evaluate('''(el) => {
        const s = getComputedStyle(el);
        return s.borderTopWidth === '1px' && s.borderTopStyle === 'solid';
    }'''), 'Application-owned card border geometry must survive class migration'
    assert page.locator('.badge').evaluate('''(el) => {
        const probe = document.createElement('span');
        probe.style.color = 'var(--fs-secondary)';
        el.append(probe);
        const s = getComputedStyle(el);
        const matches = s.color === getComputedStyle(probe).color && s.borderTopColor === s.color;
        probe.remove();
        return matches;
    }'''), 'In-progress badge and its border must use secondary'
    page.get_by_role('textbox', name='Project name').fill('preserved')
    original = page.locator('button.primary').evaluate('(el) => getComputedStyle(el).backgroundColor')
    select_option(accent, 'blue', 'Blue')
    page.wait_for_function('(original) => getComputedStyle(document.querySelector("button.primary")).backgroundColor !== original', arg=original)
    assert_theme_background()
    theme.click()
    dark_options = option_values('Themes')
    theme.press('Escape')
    assert 'catppuccin/latte' not in dark_options
    assert 'catppuccin/mocha' in dark_options
    set_mode('light')
    expect(theme).to_have_value('Catppuccin Latte')
    expect(accent).to_have_value('Blue')
    theme.click()
    light_options = option_values('Themes')
    theme.press('Escape')
    assert 'catppuccin/mocha' not in light_options
    assert 'nord/main' not in light_options
    select_option(accent, 'green', 'Green')
    select_option(theme, 'rose_pine/dawn', 'Rosé Pine Dawn')
    expect(accent).to_have_value('Default')
    select_option(accent, 'rose', 'Rose')
    set_mode('dark')
    expect(theme).to_have_value('Catppuccin Mocha')
    expect(accent).to_have_value('Blue')
    set_mode('light')
    expect(theme).to_have_value('Rosé Pine Dawn')
    expect(accent).to_have_value('Rose')
    page.wait_for_function('getComputedStyle(document.querySelector(".fs-theme")).colorScheme === "light"')
    assert_theme_background()
    set_mode('dark')
    select_option(theme, 'nord/main', 'Nord')
    expect(accent).to_have_value('Default')
    assert_theme_background()
    page.get_by_role('button', name='Reset theme').click()
    expect(theme).to_have_value('Catppuccin Mocha')
    expect(accent).to_have_value('Mauve')
    expect(mode).to_have_attribute('aria-checked', 'true')
    set_mode('light')
    expect(theme).to_have_value('Catppuccin Latte')
    expect(accent).to_have_value('Blue')
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
    expect(page.locator('.badge')).to_have_text('In progress')
    for checkbox in page.get_by_role('checkbox').all():
        checkbox.check()
    expect(page.locator('.badge')).to_have_text('Completed')
    assert page.locator('.badge').evaluate('''el => {
        const probe = document.createElement('span');
        probe.style.color = 'var(--fs-success)';
        el.append(probe);
        const matches = getComputedStyle(el).color === getComputedStyle(probe).color;
        probe.remove();
        return matches;
    }''')
    new_task.uncheck()
    expect(page.locator('.badge')).to_have_text('In progress')
    new_task.check()
    expect(page.locator('.badge')).to_have_text('Completed')
    page.get_by_role('textbox', name='New task').fill('Follow-up review')
    page.get_by_role('button', name='Add task', exact=True).click()
    expect(page.locator('.badge')).to_have_text('In progress')
    expect(page.get_by_role('button', name='Add task', exact=True)).to_be_disabled()
    page.get_by_role('button', name='Color reference', exact=True).click()
    expect(page.locator('.swatch')).to_have_count(32)
    page.get_by_role('button', name='Copy --fs-primary', exact=True).click()
    expect(page.locator('.copy-status')).to_have_text('Copied --fs-primary')
    assert page.evaluate('navigator.clipboard.readText()') == '--fs-primary'
    page.get_by_role('button', name='Copy --fs-alt-background', exact=True).focus()
    page.keyboard.press('Enter')
    expect(page.locator('.copy-status')).to_have_text('Copied --fs-alt-background')
    assert page.evaluate('navigator.clipboard.readText()') == '--fs-alt-background'
    page.evaluate("() => { window.originalWriteText = navigator.clipboard.writeText; navigator.clipboard.writeText = () => Promise.reject(new Error('Denied')); }")
    page.get_by_role('button', name='Copy --fs-error', exact=True).click()
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
        theme_box = theme.bounding_box()
        accent_box = accent.bounding_box()
        group_box = page.get_by_role('group', name='Theme and accent').bounding_box()
        assert abs(theme_box['y'] - accent_box['y']) < 1, 'Connected comboboxes must stay side by side'
        assert theme_box['x'] + theme_box['width'] <= accent_box['x'] + 1
        assert page.get_by_role('group', name='Theme and accent').locator(':scope > div').nth(1).evaluate(
            '(el) => getComputedStyle(el).borderLeftStyle === "solid"'
        )
        assert group_box['x'] <= theme_box['x'] and accent_box['x'] + accent_box['width'] <= group_box['x'] + group_box['width']
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
