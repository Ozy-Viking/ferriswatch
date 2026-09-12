#!/usr/bin/env python3
"""Verify pinned source hashes and independently reconstruct imported palette literals.

Runs offline using only Python's standard library. Role selections below document
which source fields were imported; Rust roles remain authoritative for rendering.
The source parsers never read Rust mapping expressions or execute upstream code.
"""

from pathlib import Path
import re, json, colorsys, plistlib, hashlib, math

OUT = Path(__file__).resolve().parents[1]
ROOT = OUT / "tests/fixtures/upstream"
SOURCES = {}
ENTRIES = []
FIXTURES = []


def read(repo, path):
    return (ROOT / repo / path).read_text(encoding="utf-8-sig")


def cname(name):
    if re.fullmatch("base[0-9a-fA-F]{2}", name):
        return "BASE_" + name[4:].upper()
    name = re.sub(r"([a-z0-9])([A-Z])", r"\1_\2", name)
    name = re.sub(r"([A-Za-z])([0-9])", r"\1_\2", name)
    return re.sub("[^a-zA-Z0-9]+", "_", name).strip("_").upper()


def hexval(value):
    if not isinstance(value, str):
        return None
    value = value.strip().lstrip("#")
    if re.fullmatch("[a-fA-F0-9]{3,4}", value):
        value = "".join(x * 2 for x in value)
    return (
        value.lower()
        if re.fullmatch("[a-fA-F0-9]{6}([a-fA-F0-9]{2})?", value)
        else None
    )


def jsonc(s):
    # Remove comments while respecting strings; then trailing commas outside strings.
    s = re.sub(
        r'"(?:\\.|[^"\\])*"|//[^\n]*|/\*[\s\S]*?\*/',
        lambda m: m[0] if m[0].startswith('"') else "",
        s,
    )
    return json.loads(re.sub(r",\s*([}\]])", r"\1", s))


def add(
    family, variant, label, raw, roles, accents, default, sources, light=False, note=""
):
    raw = {cname(k): hexval(v) for k, v in raw.items() if hexval(v)}
    roles = {k: cname(v) for k, v in roles.items()}
    accents = {a: cname(v) for a, v in accents.items()}
    for k, v in {**roles, **accents}.items():
        assert v in raw, (family, variant, k, v)
    assert len(raw) == len(set(raw))
    entry = dict(
        family=family,
        variant=variant,
        label=label,
        raw=raw,
        roles=roles,
        accents=accents,
        default=default,
        sources=sources,
        light=light,
        note=note,
    )
    ENTRIES.append(entry)
    return entry


def rolemap(
    bg,
    surface,
    raised,
    overlay,
    hover,
    alt,
    fg,
    muted,
    subtle,
    border,
    secondary,
    success,
    warning,
    error,
    info,
    critical=None,
    trace=None,
):
    return dict(
        background=bg,
        surface=surface,
        raised=raised,
        overlay=overlay,
        hover=hover,
        alt_background=alt,
        alt_surface=bg,
        alt_raised=surface,
        alt_overlay=overlay,
        alt_hover=hover,
        text=fg,
        text_muted=muted,
        text_subtle=subtle,
        border=border,
        border_muted=surface,
        secondary=secondary,
        success=success,
        warning=warning,
        error=error,
        critical=critical or error,
        info=info,
        trace=trace or subtle,
    )


# Solarized's original sixteen-colour table, rather than experimental Vim overrides.
path = "gimp-palette-solarized/solarized.gpl"
raw = {}
for r, g, b, name in re.findall(
    r"^\s*(\d+)\s+(\d+)\s+(\d+)\s+solarized-(\w+)", read("solarized", path), re.M
):
    raw[name] = "%02x%02x%02x" % (int(r), int(g), int(b))
for variant, light, bg, surface, fg, muted, strong in [
    ("dark", False, "base03", "base02", "base0", "base01", "base1"),
    ("light", True, "base3", "base2", "base00", "base1", "base01"),
]:
    add(
        "solarized",
        variant,
        "Solarized " + variant.title(),
        raw,
        rolemap(
            bg,
            surface,
            surface,
            surface,
            surface,
            surface,
            fg,
            strong,
            muted,
            muted,
            "cyan",
            "green",
            "yellow",
            "red",
            "blue",
            trace=muted,
        ),
        {
            x.title(): x
            for x in [
                "yellow",
                "orange",
                "red",
                "magenta",
                "violet",
                "blue",
                "cyan",
                "green",
            ]
        },
        "Blue",
        [("solarized", path)],
        light,
        "The two neutral background steps repeat for raised content and hover. Solarized has no separate critical hue; critical repeats red.",
    )

path = "src/nord.scss"
raw = {
    name: value
    for name, value in re.findall(r"\$(nord\d+):\s*(#[\da-fA-F]+)", read("nord", path))
}
add(
    "nord",
    "main",
    "Nord",
    raw,
    rolemap(
        "nord0",
        "nord1",
        "nord2",
        "nord1",
        "nord3",
        "nord1",
        "nord4",
        "nord5",
        "nord3",
        "nord3",
        "nord9",
        "nord14",
        "nord13",
        "nord11",
        "nord8",
        trace="nord10",
    ),
    {f"Nord{i}": f"nord{i}" for i in range(7, 16)},
    "Nord8",
    [("nord", path)],
    note="Polar Night supplies surfaces. Frost supplies actions and trace; critical repeats the single Aurora error red.",
)

path = "ocean.yaml"
raw = dict(
    re.findall(r"(base[0-9a-fA-F]{2}): \"([0-9a-fA-F]{6})\"", read("base16", path))
)
add(
    "base16",
    "ocean",
    "Base16 Ocean",
    raw,
    rolemap(
        "base00",
        "base01",
        "base02",
        "base01",
        "base02",
        "base01",
        "base05",
        "base04",
        "base03",
        "base03",
        "base0E",
        "base0B",
        "base0A",
        "base08",
        "base0D",
        critical="base0F",
    ),
    {f"Base{x}": f"base{x}" for x in ["08", "09", "0A", "0B", "0C", "0D", "0E", "0F"]},
    "Base0D",
    [("base16", path)],
    note="Base16 background and foreground slots retain their hexadecimal names. Base0F marks critical; Base03 marks trace.",
)

# Original Atom HSL definitions, converted to sRGB bytes with CSS rounding.
for variant in ["dark", "light"]:
    repo = "one_" + variant
    path = "styles/colors.less"
    s = read(repo, path)
    variables = {
        k: float(v.rstrip("%")) for k, v in re.findall(r"@([\w-]+):\s*([\d.]+%?);", s)
    }
    raw = {}
    for name, h, sa, l in re.findall(
        r"@([\w-]+):\s*hsl\(\s*([^,]+),\s*([^,]+),\s*([^\)]+)\)", s
    ):
        vals = [
            variables[v.strip()[1:]]
            if v.strip().startswith("@")
            else float(v.strip().rstrip("%"))
            for v in [h, sa, l]
        ]
        rgb = colorsys.hls_to_rgb(vals[0] / 360, vals[2] / 100, vals[1] / 100)
        raw[name] = "".join(f"{math.floor(c * 255 + 0.5):02x}" for c in rgb)
    # Original syntax themes have only one opaque background. Repetition is intentional.
    roles = rolemap(
        "syntax-bg",
        "syntax-bg",
        "syntax-bg",
        "syntax-bg",
        "syntax-bg",
        "syntax-bg",
        "mono-1",
        "mono-2",
        "mono-3",
        "mono-3",
        "hue-3",
        "hue-4",
        "hue-6-2",
        "hue-5",
        "hue-2",
        critical="hue-5-2",
    )
    add(
        "one",
        variant,
        "One " + variant.title(),
        raw,
        roles,
        {
            f"Hue{x}": "hue-" + x.replace("_", "-")
            for x in ["1", "2", "3", "4", "5", "6"]
        },
        "Hue2",
        [(repo, path)],
        variant == "light",
        "Original Atom syntax palette. Background roles repeat because this source has one background; border and action states supply control separation. HSL conversion is recorded by the importer.",
    )

# Ayu release with literal source colours. UI derivations are deliberately not imported as raw colours.
for variant in ["dark", "mirage", "light"]:
    repo = "ayu_release"
    path = f"src/{variant}.ts"
    s = read(repo, path)
    raw = {}
    for group, body in re.findall(r"const (common|syntax|vcs) = \{(.*?)\n\}", s, re.S):
        for name, value in re.findall(r"(\w+): _\(\'(\w+)\'\)", body):
            raw[group + "_" + name] = value
    roles = rolemap(
        "common_bg",
        "common_bg",
        "common_bg",
        "common_bg",
        "common_bg",
        "common_bg",
        "common_fg",
        "syntax_comment",
        "common_ui",
        "common_ui",
        "syntax_entity",
        "vcs_added",
        "syntax_func",
        "syntax_error",
        "syntax_tag",
        critical="vcs_removed",
    )
    add(
        "ayu",
        variant,
        "Ayu " + variant.title(),
        raw,
        roles,
        {
            x.title(): f"syntax_{x}"
            for x in [
                "tag",
                "func",
                "entity",
                "string",
                "regexp",
                "markup",
                "keyword",
                "special",
                "constant",
                "operator",
            ]
        }
        | {"Accent": "common_accent"},
        "Accent",
        [(repo, path)],
        variant == "light",
        "Ayu v7.0.1 literals preserve common/syntax/VCS vocabulary. Background roles repeat the source common background; no editor-specific generated shades are claimed.",
    )

for variant in [
    "nightfox",
    "dayfox",
    "dawnfox",
    "duskfox",
    "nordfox",
    "terafox",
    "carbonfox",
]:
    path = f"lua/nightfox/palette/{variant}.lua"
    s = read("nightfox", path)
    raw = dict(re.findall(r'(\w+)\s*=\s*(?:Shade.new\()?"(#[\da-fA-F]{6})"', s))
    if variant == "carbonfox":
        read("nightfox", "lua/nightfox/lib/color.lua")
        bg = (22, 22, 22)
        fg = (242, 244, 248)

        def brighten(rgb, amount):
            h, sa, v = colorsys.rgb_to_hsv(*(c / 255 for c in rgb))
            return "".join(
                f"{math.floor(c * 255 + 0.5):02x}"
                for c in colorsys.hsv_to_rgb(h, sa, max(0, min(1, v + amount / 100)))
            )

        raw.update(
            {f"bg{i}": brighten(bg, a) for i, a in enumerate([-4, 0, 6, 12, 24])}
        )
        raw.update({f"fg{i}": brighten(fg, a) for i, a in enumerate([6, 0, -24, -48])})
        raw["comment"] = "".join(
            f"{math.floor((a + (b - a) * 0.4) + 0.5):02x}" for a, b in zip(bg, fg)
        )
    roles = rolemap(
        "bg1",
        "bg2",
        "bg3",
        "bg0",
        "sel0",
        "bg0",
        "fg1",
        "fg2",
        "comment",
        "bg4",
        "magenta",
        "green",
        "yellow",
        "red",
        "blue",
        trace="fg3",
    )
    add(
        "nightfox",
        variant,
        variant.title(),
        raw,
        roles,
        {
            x.title(): x
            for x in [
                "red",
                "green",
                "yellow",
                "blue",
                "magenta",
                "cyan",
                "orange",
                "pink",
            ]
        },
        "Blue",
        [("nightfox", path)],
        "light = true" in s,
        "Nightfox base shades and declared UI roles. bg0 remains the floating surface, sel0 is hover, and red serves both error levels. Generated bright/dim shades are outside the raw import.",
    )

path = "lua/material/colors/init.lua"
s = read("material", path)
main = s[s.index("main = {") : s.index("---colors applied")]
base = dict(re.findall(r'^\s*(\w+)\s*=\s*"(#[\da-fA-F]{6})"', main, re.M))
for variant in ["oceanic", "palenight", "lighter"]:
    if variant == "oceanic":
        body = s[
            s.index('else vim.g.material_style = "oceanic"') : s.index(
                "---syntax colors"
            )
        ]
    else:
        start = s.index('elseif vim.g.material_style == "' + variant + '"')
        end = s.find("elseif vim.g.material_style", start + 10)
        body = s[start:end]
        if variant == "lighter":
            body = body[body.index("-- default Lighter theme style") :]
    raw = {"main_" + k: v for k, v in base.items()}
    raw.update(
        {
            group + "_" + name: v
            for group, name, v in re.findall(
                r'^\s*colors\.(\w+)\.(\w+)\s*=\s*"(#[\da-fA-F]{6})"', body, re.M
            )
        }
    )
    raw["lsp_error"] = "FF5370"
    roles = rolemap(
        "editor_bg",
        "editor_active",
        "editor_highlight",
        "editor_bg",
        "editor_selection",
        "editor_bg_alt",
        "editor_fg",
        "editor_fg_dark",
        "syntax_comments",
        "editor_border",
        "main_purple",
        "main_green",
        "main_yellow",
        "lsp_error",
        "main_paleblue",
        critical="main_red",
    )
    add(
        "material",
        variant,
        "Material " + variant.title(),
        raw,
        roles,
        {
            x.title(): "main_" + x
            for x in ["red", "green", "yellow", "blue", "cyan", "purple", "orange"]
        }
        | {"Accent": "editor_accent"},
        "Accent",
        [("material", path)],
        variant == "lighter",
        "Default style branch with high_visibility disabled. Upstream editor active/highlight/selection colours supply placement and hover. Oceanic is the named Material style, independent of Oceanic Next.",
    )

# Original named variables from Panda's own stylesheet.
path = "theme.less"
raw = dict(re.findall(r"@([\w-]+)\s*:\s*(#[\da-fA-F]+)", read("panda", path)))
add(
    "panda",
    "main",
    "Panda",
    raw,
    rolemap(
        "background",
        "background",
        "background",
        "background",
        "background",
        "background",
        "white",
        "light",
        "comment",
        "comment",
        "purple",
        "green",
        "orange",
        "error",
        "blue",
        trace="comment",
    ),
    {x.title(): x for x in ["blue", "purple", "green", "red", "orange", "pink"]},
    "Purple",
    [("panda", path)],
    note="Panda uses white for ordinary text; its variable named foreground is pink. The single opaque background repeats; orange serves warnings and error repeats as critical.",
)


# TextMate sources retain scope names and default settings as raw names.
def tm(repo, path):
    d = plistlib.loads(read(repo, path).encode())
    raw = {}
    scopes = {}
    for i, entry in enumerate(d["settings"]):
        name = entry.get("name", "default" if i == 0 else "scope_" + str(i))
        for k, v in entry["settings"].items():
            if hexval(v):
                raw[name + "_" + k] = v
        if "foreground" in entry["settings"]:
            scopes[name] = name + "_foreground"
    return raw, scopes


for variant, path in [
    ("day", "textmate/Tomorrow.tmTheme"),
    ("night", "textmate/Tomorrow-Night.tmTheme"),
]:
    raw, scopes = tm("tomorrow", path)
    # Select by source scope labels after inspecting below.
    ENTRIES.append(
        dict(
            pending="tm",
            family="tomorrow",
            variant=variant,
            path=path,
            raw=raw,
            scopes=scopes,
        )
    )
raw, scopes = tm("oceanic_next", "Oceanic Next.tmTheme")
ENTRIES.append(
    dict(
        pending="tm",
        family="oceanic_next",
        variant="main",
        path="Oceanic Next.tmTheme",
        raw=raw,
        scopes=scopes,
    )
)


# VS Code exports: preserve the declared editor keys, token names and alpha values.
def editor(repo, path):
    d = jsonc(read(repo, path))
    raw = {k: v for k, v in d.get("colors", {}).items() if hexval(v)}
    for i, t in enumerate(d.get("tokenColors", [])):
        v = t.get("settings", {}).get("foreground")
        if hexval(v):
            raw["token_" + t.get("name", str(i))] = v
    return d, raw


for family, repo, path, label in [
    ("night_owl", "night_owl", "themes/Night Owl-color-theme.json", "Night Owl"),
    ("poimandres", "poimandres", "themes/poimandres-color-theme.json", "Poimandres"),
    (
        "synthwave_84",
        "synthwave_84",
        "themes/synthwave-color-theme.json",
        "Synthwave '84",
    ),
    ("cobalt2", "cobalt2", "theme/cobalt2.json", "Cobalt2"),
    (
        "shades_of_purple",
        "shades_of_purple",
        "themes/shades-of-purple-color-theme.json",
        "Shades of Purple",
    ),
    ("horizon", "horizon", "themes/horizon.json", "Horizon"),
    (
        "monokai",
        "vscode",
        "extensions/theme-monokai/themes/monokai-color-theme.json",
        "Monokai",
    ),
    (
        "quiet_light",
        "vscode",
        "extensions/theme-quietlight/themes/quietlight-color-theme.json",
        "Quiet Light",
    ),
]:
    d, raw = editor(repo, path)
    ENTRIES.append(
        dict(
            pending="editor",
            family=family,
            variant="main",
            label=label,
            raw=raw,
            sources=[(repo, path)],
            light=family == "quiet_light",
        )
    )
for variant in ["dark", "light"]:
    base = f"extensions/theme-defaults/themes/{variant}_vs.json"
    plus = f"extensions/theme-defaults/themes/{variant}_plus.json"
    d, raw = editor("vscode", base)
    dp, rp = editor("vscode", plus)
    assert dp["include"] == f"./{variant}_vs.json"
    raw.update(rp)
    ENTRIES.append(
        dict(
            pending="editor",
            family="vscode",
            variant=variant + "_plus",
            label="VS Code " + variant.title() + "+",
            raw=raw,
            sources=[("vscode", base), ("vscode", plus)],
            light=variant == "light",
        )
    )
# Dracula YAML anchors preserve source vocabulary.
path = "src/dracula.yml"
s = read("dracula", path)
raw = dict(re.findall(r"&(\w+)\s+\'(#[\da-fA-F]+)\'", s))
add(
    "dracula",
    "main",
    "Dracula",
    raw,
    rolemap(
        "BG",
        "BGLight",
        "BGLighter",
        "BGDark",
        "SELECTION",
        "BGDarker",
        "FG",
        "FG",
        "COMMENT",
        "COMMENT",
        "PINK",
        "GREEN",
        "YELLOW",
        "RED",
        "CYAN",
        critical="COLOR9",
    ),
    {
        x.title(): x
        for x in ["cyan", "green", "orange", "pink", "purple", "red", "yellow"]
    },
    "Purple",
    [("dracula", path)],
    note="Dracula declares darker chrome and lighter control surfaces. Bright ANSI red supplies critical. The source comment colour is reserved for subtle text and trace.",
)

# Complete explicit mappings for editor exports and TextMate sources.
pending = [e for e in ENTRIES if "pending" in e]
ENTRIES[:] = [e for e in ENTRIES if "pending" not in e]
for e in pending:
    raw = e["raw"]
    family = e["family"]
    variant = e["variant"]

    def pick(*names):
        return next(n for n in names if n in raw)

    if e["pending"] == "tm":

        def scope(prefix):
            return next(v for k, v in e["scopes"].items() if k.startswith(prefix))

        accents = {
            k: scope(v)
            for k, v in {
                "Red": "Variable, String" if family == "tomorrow" else "Tag",
                "Orange": "Number, Constant",
                "Yellow": "Class, Support",
                "Green": "String, Symbols",
                "Cyan": "Operator, Misc",
                "Blue": "Function, Special",
                "Purple": "Keyword, Storage",
            }.items()
        }
        bg = "default_background"
        surf = pick("default_lineHighlight", "default_background")
        comment = scope("Comment")
        # Oceanic's line highlight is translucent and remains a hover colour, not a panel fill.
        raised = surf if family == "tomorrow" else "default_background"
        roles = rolemap(
            bg,
            raised,
            raised,
            bg,
            "default_lineHighlight",
            bg,
            "default_foreground",
            "default_foreground",
            comment,
            "default_selection",
            accents["Purple"],
            accents["Green"],
            accents["Yellow"],
            accents["Red"],
            accents["Blue"],
            critical=pick("Invalid_background", "Deleted_foreground"),
        )
        label = (
            ("Tomorrow" if variant == "day" else "Tomorrow Night")
            if family == "tomorrow"
            else "Oceanic Next"
        )
        add(
            family,
            variant,
            label,
            raw,
            roles,
            accents,
            "Blue",
            [(family, e["path"])],
            variant == "day",
            "Original TextMate settings and named scopes. Line highlight supplies hover and selection supplies borders; floating content repeats the canvas when upstream has no distinct popup fill.",
        )
        continue
    if family == "vscode":
        bg = "editor.background"
        fg = "editor.foreground"
        surf = "editorGroupHeader.tabsBackground"
        over = pick("menu.background", "editorSuggestWidget.background")
        muted = "sideBarTitle.foreground"
        border = pick("menu.border", "widget.border")
        accents = {
            "Blue": "activityBarBadge.background",
            "Purple": "token_Control flow / Special keywords",
            "Cyan": "token_Types declaration and references",
            "Function": "token_Function declarations",
            "Variable": "token_Variable and parameter name",
        }
        roles = rolemap(
            bg,
            surf,
            over,
            over,
            pick("list.hoverBackground", "list.dropBackground"),
            over,
            fg,
            muted,
            "input.placeholderForeground",
            border,
            accents["Purple"],
            "ports.iconRunningProcessForeground",
            "token_Function declarations",
            pick("token_11", "token_9"),
            "activityBarBadge.background",
        )
        default = "Blue"
    elif family == "quiet_light":
        accents = {
            k: "token_" + v
            for k, v in {
                "Blue": "Keywords",
                "Purple": "Types",
                "Green": "Strings",
                "Red": "Functions",
                "Orange": "Numbers, Characters",
            }.items()
        }
        roles = rolemap(
            "editor.background",
            "sideBar.background",
            "sideBarSectionHeader.background",
            "dropdown.background",
            "list.hoverBackground",
            "activityBar.background",
            "token_0",
            "token_Operators",
            "token_Comments",
            "panelSection.border",
            "button.background",
            "token_Strings",
            "list.warningForeground",
            "token_Invalid",
            "token_Keywords",
            critical="token_Invalid - Illegal",
        )
        default = "Purple"
    else:
        accents = {
            x: "terminal.ansi" + x
            for x in ["Red", "Green", "Yellow", "Blue", "Magenta", "Cyan"]
        }
        if family == "monokai":
            accents = {x: "terminal.ansiBright" + x for x in accents}
        bg = "editor.background"
        fg = pick("editor.foreground", "foreground")
        surf = pick("sideBar.background", bg)
        over = pick("editorWidget.background", surf)
        hover = pick("list.hoverBackground", "editor.lineHighlightBackground", surf)
        raised = pick("dropdown.background", "button.background", over)
        # Use opaque source colours for placement; alpha stays in hover where specified.
        if len(hexval(raw[raised])) == 8:
            raised = surf
        muted = pick("descriptionForeground", "foreground", fg)
        subtle = pick("editorLineNumber.foreground", "descriptionForeground", muted)
        border = pick(
            "editorWidget.border",
            "panel.border",
            "sideBar.border",
            "focusBorder",
            hover,
        )
        error = pick("editorError.foreground", "terminal.ansiRed")
        warning = pick("editorWarning.foreground", "terminal.ansiYellow")
        info = pick("editorInfo.foreground", "terminal.ansiBlue")
        # Syntax diagnostics in these sources use low-contrast squiggle colours or green warnings.
        if family == "cobalt2":
            error = "terminal.ansiRed"
        if family in ["horizon", "synthwave_84"]:
            warning = "terminal.ansiYellow"
        roles = rolemap(
            bg,
            surf,
            raised,
            over,
            hover,
            surf,
            fg,
            muted,
            subtle,
            border,
            accents["Magenta"],
            accents["Green"],
            warning,
            error,
            info,
            critical=pick("terminal.ansiBrightRed", "terminal.ansiRed"),
        )
        default = {
            "synthwave_84": "Magenta",
            "shades_of_purple": "Yellow",
            "monokai": "Green",
            "poimandres": "Green",
            "horizon": "Magenta",
        }.get(family, "Blue")
    note = "Editor UI keys supply surfaces and source alpha is retained. Terminal accents supply filled actions. Critical uses bright red; trace uses subdued source text. Roles that share an upstream colour intentionally repeat."
    if family == "vscode":
        note = "The Plus theme is merged over its included VS theme. Source menu and tabs supply surfaces, token colours supply named accents, and no Modern theme is substituted."
    if family == "monokai":
        note = "VS Code’s bundled original-Monokai-based theme, not Monokai Pro. Bright terminal colours preserve the familiar accent palette. Editor and sidebar keys supply surfaces."
    if family == "quiet_light":
        note = "Quiet Light is its own family. Source syntax foreground and named token colours supply readable statuses; pale validation borders are not used as status text."
    add(
        family,
        variant,
        e["label"],
        raw,
        roles,
        accents,
        default,
        e["sources"],
        e["light"],
        note,
    )

# GitHub default themes: Primer 7.10.0 scales, semantic references, and explicit theme overrides.
for variant in ["dark", "light"]:
    path = f"data/colors/themes/{variant}.ts"
    s = read("primer", path)
    raw = {}
    for key, body in re.findall(r"(\w+):\s*\[([^\]]+)\]", s, re.S):
        for i, value in enumerate(re.findall(r"'(#[\da-fA-F]{6})'", body)):
            raw[f"scale_{key}_{i}"] = value
    for key, value in re.findall(r"(black|white): '(#[\da-fA-F]{6})'", s):
        raw["scale_" + key] = value
    globalpath = f"data/colors/vars/global_{variant}.ts"
    glob = read("primer", globalpath)
    for group, body in re.findall(r"^  (\w+): \{(.*?)^  \}", glob, re.M | re.S):
        for name, key in re.findall(r"(\w+): get\('scale\.([\w.]+)'\)", body):
            raw[group + "_" + name] = raw["scale_" + key.replace(".", "_")]
    read("primer", f"data/colors/vars/component_{variant}.ts")
    theme = read("github", "src/colors.js")
    read("github", "src/theme.js")
    read("github", "package.json")
    body = theme.split(f'case "{variant}":', 1)[1].split("return ", 1)[0]
    for group, name, value in re.findall(
        rf'{variant}Colors\.(\w+)\.(\w+) = "(#[\da-fA-F]+)"', body
    ):
        raw[group + "_" + name] = value
    accents = {
        x.title(): "scale_" + x + ("_5" if variant == "light" else "_3")
        for x in ["blue", "green", "yellow", "orange", "red", "purple", "pink"]
    }
    roles = rolemap(
        "canvas_default",
        "canvas_subtle",
        "canvas_overlay",
        "canvas_overlay",
        "border_default",
        "canvas_inset",
        "fg_default",
        "fg_muted",
        "fg_subtle",
        "border_default",
        "accent_emphasis",
        "success_fg",
        "attention_fg",
        "danger_fg",
        "accent_fg",
        critical="danger_emphasis",
    )
    sources = [
        ("github", "src/theme.js"),
        ("github", "src/colors.js"),
        ("github", "package.json"),
        ("primer", path),
        ("primer", globalpath),
        ("primer", f"data/colors/vars/component_{variant}.ts"),
    ]
    add(
        "github",
        variant,
        "GitHub " + variant.title(),
        raw,
        roles,
        accents,
        "Blue",
        sources,
        variant == "light",
        "GitHub default theme with Primer 7.10.0 and explicit fg/accent overrides. Canvas inset/subtle/overlay retain their meanings. Actions use the selected curated scale hue; status hues remain independent.",
    )

# JetBrains editor inheritance and UI palette are imported separately, then mapped explicitly.
import xml.etree.ElementTree as ET

common = "platform/platform-resources/src/DefaultColorSchemesManager.xml"
tree = ET.fromstring(read("jetbrains", common))


def scheme_raw(scheme, base):
    raw = base.copy()
    for opt in scheme.findall("./colors/option"):
        if hexval(opt.get("value")):
            raw["editor_" + opt.get("name")] = opt.get("value")
    for opt in scheme.findall("./attributes/option"):
        for value in opt.findall("./value/option"):
            if value.get("name") in [
                "FOREGROUND",
                "BACKGROUND",
                "EFFECT_COLOR",
            ] and value.get("value"):
                v = value.get("value")
                v = v.zfill(6) if len(v) < 6 else v
                if hexval(v):
                    raw["editor_" + opt.get("name") + "_" + value.get("name")] = v
    return raw


base = scheme_raw(tree.find("scheme[@name='Default']"), {})
darcula = scheme_raw(tree.find("scheme[@name='Darcula']"), base)
ui_path = "platform/platform-resources/src/themes/darcula.theme.json"
ui = json.loads(read("jetbrains", ui_path))


def flatten(value, prefix=""):
    result = {}
    for k, v in value.items():
        key = prefix + "_" + k if prefix else k
        if isinstance(v, dict):
            result.update(flatten(v, key))
        else:
            result[key] = v
    return result


ui_raw = {k: v for k, v in flatten(ui["ui"], "ui").items() if hexval(v)}
raw = darcula | ui_raw
# Print keys once to select source diagnostic foregrounds.
for variant in ["darcula", "dark"]:
    if variant == "dark":
        darkpath = "platform/platform-resources/src/themes/expUI/expUI_darkScheme.xml"
        darkui = "platform/platform-resources/src/themes/expUI/expUI_dark.theme.json"
        raw = scheme_raw(ET.fromstring(read("jetbrains", darkpath)), darcula)
        d = json.loads(read("jetbrains", darkui))
        raw.update(d["colors"])
        for k, v in flatten(d["ui"], "ui").items():
            if v in d["colors"] if isinstance(v, str) else False:
                raw[k] = d["colors"][v]
            elif hexval(v):
                raw[k] = v
        roles = rolemap(
            "editor_TEXT_BACKGROUND",
            "Gray2",
            "Gray3",
            "Gray2",
            "ui_ActionButton_hoverBackground",
            "Gray1",
            "editor_TEXT_FOREGROUND",
            "Gray11",
            "Gray8",
            "Gray4",
            "Gray3",
            "Green7",
            "Yellow7",
            "Red7",
            "Blue9",
            critical="Red8",
            trace="Gray8",
        )
        accents = {
            x: x + "6" for x in ["Blue", "Green", "Yellow", "Red", "Orange", "Purple"]
        }
        default = "Blue"
        sources = [
            ("jetbrains", common),
            ("jetbrains", ui_path),
            ("jetbrains", darkpath),
            ("jetbrains", darkui),
        ]
    else:
        roles = rolemap(
            "editor_TEXT_BACKGROUND",
            "ui_*_background",
            "ui_TextField_background",
            "ui_Popup_background",
            "ui_ActionButton_hoverBackground",
            "ui_*_background",
            "editor_TEXT_FOREGROUND",
            "ui_Label_foreground",
            "ui_Component_infoForeground",
            "ui_Component_borderColor",
            "ui_Button_startBackground",
            "editor_CONSOLE_GREEN_OUTPUT_FOREGROUND",
            "editor_CONSOLE_YELLOW_OUTPUT_FOREGROUND",
            "editor_CONSOLE_RED_OUTPUT_FOREGROUND",
            "editor_CONSOLE_BLUE_OUTPUT_FOREGROUND",
            critical="editor_ERRORS_ATTRIBUTES_EFFECT_COLOR",
        )
        accents = {
            x: "editor_CONSOLE_" + x.upper() + "_OUTPUT_FOREGROUND"
            for x in ["Blue", "Green", "Yellow", "Red", "Magenta", "Cyan"]
        }
        default = "Blue"
        sources = [("jetbrains", common), ("jetbrains", ui_path)]
    add(
        "jetbrains",
        variant,
        "Darcula" if variant == "darcula" else "JetBrains Dark",
        raw,
        roles,
        accents,
        default,
        sources,
        note="Editor scheme inherits Default → Darcula → Dark as applicable. UI surfaces use each theme’s declared palette. Translucent Dark action hover is retained; Darcula has its own editor background and console colours.",
    )


root = OUT
src = ROOT


def cn(s):
    s = re.sub(r"([a-z0-9])([A-Z])", r"\1_\2", s)
    s = re.sub(r"([a-zA-Z])([0-9])", r"\1_\2", s)
    return s.strip("_").upper()


def lua(s):
    s = re.sub(r"^\s*\w+ = \{[^{}]*\},?", "", s, flags=re.M)
    return {
        cn(k): v.lower() + "ff"
        for k, v in re.findall(r'^\s*(\w+)\s*=\s*"#([\da-fA-F]{6})"', s, re.M)
    }


expected = {}
c = json.loads((src / "catppuccin/palette.json").read_text())
for v in ["latte", "frappe", "macchiato", "mocha"]:
    expected["catppuccin/" + v] = {
        cn(k): x["hex"][1:] + "ff" for k, x in c[v]["colors"].items()
    }
k = lua((src / "kanagawa/lua/kanagawa/colors.lua").read_text())
for v in ["wave", "dragon", "lotus"]:
    expected["kanagawa/" + v] = k
s = (src / "rose_pine/lua/rose-pine/palette.lua").read_text()
for v in ["main", "moon", "dawn"]:
    body = re.search(r"\b" + v + r" = \{(.*?)\n\s*\}", s, re.S)[1]
    expected["rose_pine/" + v] = lua(body)
base = lua((src / "tokyo_night/lua/tokyonight/colors/storm.lua").read_text())
for v in ["storm", "night", "moon", "day"]:
    path = (
        "extras/lua/tokyonight_day.lua"
        if v == "day"
        else f"lua/tokyonight/colors/{v}.lua"
    )
    values = lua((src / "tokyo_night" / path).read_text().split("\n}", 1)[0])
    if v == "night":
        values = base | values
    expected["tokyo_night/" + v] = values
s = (src / "gruvbox/colors/gruvbox.vim").read_text()
raw = dict(re.findall(r"let s:gb\.(\w+)\s*=\s*\['#([\da-fA-F]{6})'", s))
for v in ["dark", "light"]:
    for con in ["soft", "medium", "hard"]:
        d = {}
        for i in range(5):
            d[f"BG_{i}"] = raw[
                v + str(i) + ("_" + con if i == 0 and con != "medium" else "")
            ]
            d[f"FG_{i}"] = raw[("light" if v == "dark" else "dark") + str(i)]
        for color in ["red", "green", "yellow", "blue", "purple", "aqua", "orange"]:
            d[color.upper()] = raw[("bright_" if v == "dark" else "faded_") + color]
        d["GRAY"] = raw["gray_245" if v == "dark" else "gray_244"]
        expected[f"gruvbox/{v}/{con}"] = {k: x.lower() + "ff" for k, x in d.items()}
s = (src / "everforest/autoload/everforest.vim").read_text()
bgs = re.findall(r"let palette1 = \{(.*?)\\ \}", s, re.S)
fgs = re.findall(r"let palette2 = \{(.*?)\\ \}", s, re.S)
assert len(bgs) == 6 and len(fgs) == 2, (len(bgs), len(fgs))


def vim(body):
    return {
        cn(k): x.lower() + "ff"
        for k, x in re.findall(r"'(\w+)':\s*\['#([\da-fA-F]{6})'", body)
    }


for i, con in enumerate(["hard", "medium", "soft"]):
    for j, v in enumerate(["dark", "light"]):
        expected[f"everforest/{v}/{con}"] = vim(bgs[i * 2 + j]) | vim(fgs[j])

for entry in ENTRIES:
    if entry["family"] == "vscode" and entry["variant"] == "dark_plus":
        entry["roles"]["error"] = entry["roles"]["critical"] = "TOKEN_9"

all_expected = {e["family"] + "/" + e["variant"]: e["raw"] for e in ENTRIES}
all_expected.update(
    {
        k: {n: v[:6] if v.endswith("ff") else v for n, v in d.items()}
        for k, d in expected.items()
    }
)
manifest = json.loads((ROOT / "manifest.json").read_text())
for source in manifest:
    data = (OUT / source["fixture"]).read_bytes()
    assert hashlib.sha256(data).hexdigest() == source["sha256"], source["fixture"]
count = 0
seen = set()
for line in (OUT / "tests/fixtures/palettes.tsv").read_text().splitlines():
    if not line or line.startswith("#"):
        continue
    theme, name, rgba = line.split("\t")
    raw = all_expected[theme][name]
    actual = raw + "ff" if len(raw) == 6 else raw
    assert rgba.lower() == actual.lower(), (theme, name, rgba, actual)
    seen.add(theme)
    count += 1
assert seen == set(all_expected), seen ^ set(all_expected)
print(
    f"Verified {count} imported literals for {len(seen)} palettes and {len(manifest)} source hashes"
)

for line in (OUT / "tests/fixtures/semantic.tsv").read_text().splitlines():
    if not line or line.startswith("#"):
        continue
    theme, *values = line.split("\t")
    entry = next(e for e in ENTRIES if e["family"] + "/" + e["variant"] == theme)
    roles = entry["roles"]
    keys = [
        roles["background"],
        roles["text"],
        entry["accents"][entry["default"]],
        roles["success"],
        roles["warning"],
        roles["error"],
    ]
    expected = [entry["raw"][key] for key in keys]
    assert values == [v + "ff" if len(v) == 6 else v for v in expected], theme
print("Verified semantic source anchors for all 38 new palettes")

source_accents = {}
for entry in ENTRIES:
    for name, key in entry["accents"].items():
        accent_id = (
            "base_" + name[4:].lower()
            if name.startswith("Base")
            else cname(name).lower()
        )
        value = entry["raw"][key]
        source_accents[(entry["family"] + "/" + entry["variant"], accent_id)] = (
            value + "ff" if len(value) == 6 else value
        )
fixture_accents = {}
for line in (OUT / "tests/fixtures/accents.tsv").read_text().splitlines():
    if line and not line.startswith("#"):
        theme, accent, value = line.split("\t")
        fixture_accents[(theme, accent)] = value
assert source_accents == fixture_accents
print(
    f"Verified {len(source_accents)} named accent assignments for the 38 new palettes"
)
