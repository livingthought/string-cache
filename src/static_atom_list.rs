// Copyright 2014 The Servo Project Developers. See the
// COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

pub static ATOMS: &'static [&'static str] = &[

    // The order is not preserved by phf.

    "a",
    "address",
    "applet",
    "area",
    "article",
    "aside",
    "b",
    "base",
    "basefont",
    "bgsound",
    "big",
    "blockquote",
    "body",
    "br",
    "button",
    "caption",
    "col",
    "colgroup",
    "dd",
    "dt",
    "embed",
    "form",
    "frame",
    "frameset",
    "h1",
    "h2",
    "h3",
    "h4",
    "h5",
    "h6",
    "head",
    "html",
    "input",
    "li",
    "link",
    "marquee",
    "meta",
    "noframes",
    "noscript",
    "object",
    "optgroup",
    "option",
    "param",
    "plaintext",
    "pre",
    "rp",
    "rt",
    "script",
    "select",
    "source",
    "style",
    "svg",
    "table",
    "tbody",
    "td",
    "template",
    "textarea",
    "tfoot",
    "th",
    "thead",
    "title",
    "tr",
    "track",
    "xmp",

    "",

    "*",

    // XML namespaces known to the HTML syntax spec
    "http://www.w3.org/1999/xhtml",
    "http://www.w3.org/XML/1998/namespace",
    "http://www.w3.org/2000/xmlns/",
    "http://www.w3.org/1999/xlink",
    "http://www.w3.org/2000/svg",
    "http://www.w3.org/1998/Math/MathML",

    "abbr",
    "abs",
    "accent",
    "accent-height",
    "accentunder",
    "accept",
    "accept-charset",
    "accesskey",
    "accumulate",
    "acronym",
    "action",
    "actiontype",
    "active",
    "actuate",
    "additive",
    "align",
    "alignment-baseline",
    "alignmentscope",
    "alink",
    "alphabetic",
    "alt",
    "altglyph",
    "altGlyph",
    "altglyphdef",
    "altGlyphDef",
    "altglyphitem",
    "altGlyphItem",
    "altimg",
    "alttext",
    "amplitude",
    "and",
    "animate",
    "animatecolor",
    "animateColor",
    "animatemotion",
    "animateMotion",
    "animatetransform",
    "animateTransform",
    "animation",
    "annotation",
    "annotation-xml",
    "apply",
    "approx",
    "arabic-form",
    "arccos",
    "arccosh",
    "arccot",
    "arccoth",
    "arccsc",
    "arccsch",
    "archive",
    "arcrole",
    "arcsec",
    "arcsech",
    "arcsin",
    "arcsinh",
    "arctan",
    "arctanh",
    "arg",
    "aria-activedescendant",
    "aria-atomic",
    "aria-autocomplete",
    "aria-busy",
    "aria-channel",
    "aria-checked",
    "aria-controls",
    "aria-datatype",
    "aria-describedby",
    "aria-disabled",
    "aria-dropeffect",
    "aria-expanded",
    "aria-flowto",
    "aria-grab",
    "aria-haspopup",
    "aria-hidden",
    "aria-invalid",
    "aria-labelledby",
    "aria-level",
    "aria-live",
    "aria-multiline",
    "aria-multiselectable",
    "aria-owns",
    "aria-posinset",
    "aria-pressed",
    "aria-readonly",
    "aria-relevant",
    "aria-required",
    "aria-secret",
    "aria-selected",
    "aria-setsize",
    "aria-sort",
    "aria-templateid",
    "aria-valuemax",
    "aria-valuemin",
    "aria-valuenow",
    "ascent",
    "async",
    "attributename",
    "attributeName",
    "attributetype",
    "attributeType",
    "audio",
    "autocomplete",
    "autofocus",
    "autoplay",
    "autosubmit",
    "axis",
    "azimuth",
    "background",
    "background-attachment",
    "background-clip",
    "background-color",
    "background-image",
    "background-origin",
    "background-position",
    "background-repeat",
    "background-size",
    "basefrequency",
    "baseFrequency",
    "baseline",
    "baseline-shift",
    "baseprofile",
    "baseProfile",
    "bbox",
    "bdi",
    "bdo",
    "beforeunload",
    "begin",
    "bevelled",
    "bgcolor",
    "bias",
    "blink",
    "border",
    "border-bottom",
    "border-bottom-color",
    "border-bottom-left-radius",
    "border-bottom-right-radius",
    "border-bottom-style",
    "border-bottom-width",
    "border-color",
    "border-left",
    "border-left-color",
    "border-left-style",
    "border-left-width",
    "border-radius",
    "border-right",
    "border-right-color",
    "border-right-style",
    "border-right-width",
    "border-style",
    "border-top",
    "border-top-color",
    "border-top-left-radius",
    "border-top-right-radius",
    "border-top-style",
    "border-top-width",
    "border-width",
    "bottom",
    "bvar",
    "by",
    "calcmode",
    "calcMode",
    "canvas",
    "cap-height",
    "card",
    "cartesianproduct",
    "ceiling",
    "cellpadding",
    "cellspacing",
    "center",
    "change",
    "char",
    "charoff",
    "charset",
    "checkbox",
    "checked",
    "ci",
    "circle",
    "cite",
    "class",
    "classid",
    "clear",
    "click",
    "clip",
    "clip-path",
    "clippath",
    "clipPath",
    "clippathunits",
    "clipPathUnits",
    "clip-rule",
    "close",
    "closure",
    "cn",
    "code",
    "codebase",
    "codetype",
    "codomain",
    "color",
    "color-interpolation",
    "color-interpolation-filters",
    "color-profile",
    "color-rendering",
    "cols",
    "colspan",
    "columnalign",
    "column-count",
    "columnlines",
    "columns",
    "columnspacing",
    "columnspan",
    "column-width",
    "columnwidth",
    "compact",
    "complexes",
    "compose",
    "condition",
    "conjugate",
    "content",
    "contenteditable",
    "contentscripttype",
    "contentScriptType",
    "contentstyletype",
    "contentStyleType",
    "contextmenu",
    "controls",
    "coords",
    "cos",
    "cosh",
    "cot",
    "coth",
    "crossorigin",
    "csc",
    "csch",
    "csymbol",
    "curl",
    "cursor",
    "cx",
    "cy",
    "d",
    "data",
    "datafld",
    "dataformatas",
    "datalist",
    "datasrc",
    "datatemplate",
    "date",
    "datetime",
    "datetime-local",
    "declare",
    "default",
    "defer",
    "definition-src",
    "definitionurl",
    "definitionURL",
    "defs",
    "degree",
    "del",
    "depth",
    "desc",
    "descent",
    "details",
    "determinant",
    "dfn",
    "dialog",
    "diff",
    "diffuseconstant",
    "diffuseConstant",
    "dir",
    "direction",
    "disabled",
    "discard",
    "display",
    "displaystyle",
    "div",
    "divergence",
    "divide",
    "divisor",
    "dl",
    "domain",
    "domainofapplication",
    "DOMContentLoaded",
    "dominant-baseline",
    "draggable",
    "dur",
    "dx",
    "dy",
    "edge",
    "edgemode",
    "edgeMode",
    "elevation",
    "ellipse",
    "em",
    "email",
    "emptyset",
    "enable-background",
    "encoding",
    "enctype",
    "end",
    "eq",
    "equalcolumns",
    "equalrows",
    "equivalent",
    "error",
    "eulergamma",
    "event",
    "exists",
    "exp",
    "exponent",
    "exponentiale",
    "externalresourcesrequired",
    "externalResourcesRequired",
    "face",
    "factorial",
    "factorof",
    "false",
    "feblend",
    "feBlend",
    "fecolormatrix",
    "feColorMatrix",
    "fecomponenttransfer",
    "feComponentTransfer",
    "fecomposite",
    "feComposite",
    "feconvolvematrix",
    "feConvolveMatrix",
    "fediffuselighting",
    "feDiffuseLighting",
    "fedisplacementmap",
    "feDisplacementMap",
    "fedistantlight",
    "feDistantLight",
    "fedropshadow",
    "feDropShadow",
    "feflood",
    "feFlood",
    "fefunca",
    "feFuncA",
    "fefuncb",
    "feFuncB",
    "fefuncg",
    "feFuncG",
    "fefuncr",
    "feFuncR",
    "fegaussianblur",
    "feGaussianBlur",
    "feimage",
    "feImage",
    "femerge",
    "feMerge",
    "femergenode",
    "feMergeNode",
    "femorphology",
    "feMorphology",
    "fence",
    "feoffset",
    "feOffset",
    "fepointlight",
    "fePointLight",
    "fespecularlighting",
    "feSpecularLighting",
    "fespotlight",
    "feSpotLight",
    "fetile",
    "feTile",
    "feturbulence",
    "feTurbulence",
    "fieldset",
    "figcaption",
    "figure",
    "file",
    "fill",
    "fill-opacity",
    "fill-rule",
    "filter",
    "filterres",
    "filterRes",
    "filterunits",
    "filterUnits",
    "flood-color",
    "flood-opacity",
    "floor",
    "fn",
    "font",
    "font-face",
    "font-face-format",
    "font-face-name",
    "font-face-src",
    "font-face-uri",
    "font-family",
    "fontfamily",
    "font-size",
    "fontsize",
    "font-size-adjust",
    "font-stretch",
    "font-style",
    "fontstyle",
    "font-variant",
    "font-weight",
    "fontweight",
    "footer",
    "for",
    "forall",
    "foreignobject",
    "foreignObject",
    "formaction",
    "format",
    "formenctype",
    "formmethod",
    "formnovalidate",
    "formtarget",
    "frameborder",
    "framespacing",
    "from",
    "fx",
    "fy",
    "g",
    "g1",
    "g2",
    "gcd",
    "geq",
    "glyph",
    "glyph-name",
    "glyph-orientation-horizontal",
    "glyph-orientation-vertical",
    "glyphref",
    "glyphRef",
    "grad",
    "gradienttransform",
    "gradientTransform",
    "gradientunits",
    "gradientUnits",
    "groupalign",
    "gt",
    "handler",
    "hanging",
    "header",
    "headers",
    "height",
    "hgroup",
    "hidden",
    "hidefocus",
    "high",
    "hkern",
    "horiz-adv-x",
    "horiz-origin-x",
    "horiz-origin-y",
    "hr",
    "href",
    "hreflang",
    "hspace",
    "http-equiv",
    "i",
    "icon",
    "id",
    "ident",
    "ideographic",
    "iframe",
    "image",
    "image-rendering",
    "imaginary",
    "imaginaryi",
    "img",
    "implies",
    "in",
    "in2",
    "index",
    "infinity",
    "inputmode",
    "ins",
    "int",
    "integers",
    "intercept",
    "intersect",
    "interval",
    "inverse",
    "irrelevant",
    "isindex",
    "ismap",
    "k",
    "k1",
    "k2",
    "k3",
    "k4",
    "kbd",
    "kernelmatrix",
    "kernelMatrix",
    "kernelunitlength",
    "kernelUnitLength",
    "kerning",
    "keydown",
    "keygen",
    "keypoints",
    "keyPoints",
    "keysplines",
    "keySplines",
    "keytimes",
    "keyTimes",
    "label",
    "lambda",
    "lang",
    "language",
    "laplacian",
    "largeop",
    "lcm",
    "left",
    "legend",
    "lengthadjust",
    "lengthAdjust",
    "leq",
    "letter-spacing",
    "lighting-color",
    "limit",
    "limitingconeangle",
    "limitingConeAngle",
    "line",
    "lineargradient",
    "linearGradient",
    "linebreak",
    "line-height",
    "linethickness",
    "list",
    "listener",
    "listing",
    "list-style",
    "list-style-image",
    "list-style-position",
    "list-style-type",
    "ln",
    "local",
    "log",
    "logbase",
    "longdesc",
    "loop",
    "low",
    "lowlimit",
    "lowsrc",
    "lquote",
    "lspace",
    "lt",
    "macros",
    "maction",
    "main",
    "maligngroup",
    "malignmark",
    "manifest",
    "map",
    "margin",
    "margin-bottom",
    "marginheight",
    "margin-left",
    "margin-right",
    "margin-top",
    "marginwidth",
    "mark",
    "marker",
    "marker-end",
    "markerheight",
    "markerHeight",
    "marker-mid",
    "marker-start",
    "markerunits",
    "markerUnits",
    "markerwidth",
    "markerWidth",
    "mask",
    "maskcontentunits",
    "maskContentUnits",
    "maskunits",
    "maskUnits",
    "math",
    "mathbackground",
    "mathcolor",
    "mathematical",
    "mathsize",
    "mathvariant",
    "matrix",
    "matrixrow",
    "max",
    "max-height",
    "maxlength",
    "maxsize",
    "max-width",
    "mean",
    "media",
    "median",
    "mediummathspace",
    "menclose",
    "menu",
    "menuitem",
    "merror",
    "metadata",
    "meter",
    "method",
    "mfenced",
    "mfrac",
    "mglyph",
    "mi",
    "min",
    "min-height",
    "minsize",
    "minus",
    "min-width",
    "missing-glyph",
    "mlabeledtr",
    "mmultiscripts",
    "mn",
    "mo",
    "mode",
    "month",
    "moment",
    "momentabout",
    "movablelimits",
    "mouseover",
    "mover",
    "mozbrowser",
    "mpadded",
    "mpath",
    "mphantom",
    "mprescripts",
    "mroot",
    "mrow",
    "ms",
    "mspace",
    "msqrt",
    "mstyle",
    "msub",
    "msubsup",
    "msup",
    "mtable",
    "mtd",
    "mtext",
    "mtr",
    "multicol",
    "multiple",
    "munder",
    "munderover",
    "name",
    "nargs",
    "naturalnumbers",
    "nav",
    "neq",
    "nest",
    "nextid",
    "nobr",
    "noembed",
    "nohref",
    "none",
    "noresize",
    "noshade",
    "not",
    "notanumber",
    "notation",
    "notin",
    "notprsubset",
    "notsubset",
    "novalidate",
    "nowrap",
    "number",
    "numoctaves",
    "numOctaves",
    "occurrence",
    "offset",
    "ol",
    "onabort",
    "onactivate",
    "onafterprint",
    "onafterupdate",
    "onbefordeactivate",
    "onbeforeactivate",
    "onbeforecopy",
    "onbeforecut",
    "onbeforeeditfocus",
    "onbeforepaste",
    "onbeforeprint",
    "onbeforeunload",
    "onbeforeupdate",
    "onbegin",
    "onblur",
    "onbounce",
    "oncellchange",
    "onchange",
    "onclick",
    "oncontextmenu",
    "oncontrolselect",
    "oncopy",
    "oncut",
    "ondataavailable",
    "ondatasetchanged",
    "ondatasetcomplete",
    "ondblclick",
    "ondeactivate",
    "ondrag",
    "ondragdrop",
    "ondragend",
    "ondragenter",
    "ondragleave",
    "ondragover",
    "ondragstart",
    "ondrop",
    "onend",
    "onerror",
    "onerrorupdate",
    "onfilterchange",
    "onfinish",
    "onfocus",
    "onfocusin",
    "onfocusout",
    "onformchange",
    "onforminput",
    "onhashchange",
    "onhelp",
    "oninput",
    "oninvalid",
    "onkeydown",
    "onkeypress",
    "onkeyup",
    "onlanguagechange",
    "onload",
    "onlosecapture",
    "onmessage",
    "onmousedown",
    "onmouseenter",
    "onmouseleave",
    "onmousemove",
    "onmouseout",
    "onmouseover",
    "onmouseup",
    "onmousewheel",
    "onmove",
    "onmoveend",
    "onmovestart",
    "onoffline",
    "ononline",
    "onpagehide",
    "onpageshow",
    "onpaste",
    "onpopstate",
    "onpropertychange",
    "onreadystatechange",
    "onrepeat",
    "onreset",
    "onresize",
    "onrowenter",
    "onrowexit",
    "onrowsdelete",
    "onrowsinserted",
    "onscroll",
    "onselect",
    "onselectstart",
    "onstart",
    "onstop",
    "onstorage",
    "onsubmit",
    "onunload",
    "onzoom",
    "opacity",
    "open",
    "operator",
    "optimum",
    "or",
    "order",
    "orient",
    "orientation",
    "origin",
    "other",
    "otherwise",
    "outerproduct",
    "outline",
    "outline-color",
    "outline-style",
    "outline-width",
    "output",
    "overflow",
    "overflow-wrap",
    "overflow-x",
    "overflow-y",
    "overline-position",
    "overline-thickness",
    "p",
    "padding",
    "padding-bottom",
    "padding-left",
    "padding-right",
    "padding-top",
    "panose-1",
    "partialdiff",
    "password",
    "path",
    "pathlength",
    "pathLength",
    "pattern",
    "patterncontentunits",
    "patternContentUnits",
    "patterntransform",
    "patternTransform",
    "patternunits",
    "patternUnits",
    "pi",
    "piece",
    "piecewise",
    "ping",
    "placeholder",
    "plus",
    "pointer-events",
    "points",
    "pointsatx",
    "pointsAtX",
    "pointsaty",
    "pointsAtY",
    "pointsatz",
    "pointsAtZ",
    "polygon",
    "polyline",
    "position",
    "poster",
    "power",
    "prefetch",
    "preservealpha",
    "preserveAlpha",
    "preserveaspectratio",
    "preserveAspectRatio",
    "primes",
    "primitiveunits",
    "primitiveUnits",
    "product",
    "profile",
    "progress",
    "prompt",
    "prsubset",
    "q",
    "quotient",
    "r",
    "radialgradient",
    "radialGradient",
    "radio",
    "radiogroup",
    "radius",
    "rationals",
    "rb",
    "readonly",
    "readystatechange",
    "real",
    "reals",
    "rect",
    "refx",
    "refX",
    "refy",
    "refY",
    "rel",
    "reln",
    "rem",
    "rendering-intent",
    "repeat",
    "repeatcount",
    "repeatCount",
    "repeatdur",
    "repeatDur",
    "repeat-max",
    "repeat-min",
    "repeat-start",
    "repeat-template",
    "replace",
    "required",
    "requiredextensions",
    "requiredExtensions",
    "requiredfeatures",
    "requiredFeatures",
    "reset",
    "restart",
    "result",
    "rev",
    "right",
    "role",
    "root",
    "rotate",
    "rowalign",
    "rowlines",
    "rows",
    "rowspacing",
    "rowspan",
    "rquote",
    "rspace",
    "rtc",
    "ruby",
    "rule",
    "rules",
    "rx",
    "ry",
    "s",
    "samp",
    "sandbox",
    "scalarproduct",
    "scale",
    "scheme",
    "scope",
    "scoped",
    "scriptlevel",
    "scriptminsize",
    "scriptsizemultiplier",
    "scrolldelay",
    "scrolling",
    "sdev",
    "seamless",
    "search",
    "sec",
    "sech",
    "section",
    "seed",
    "selected",
    "selection",
    "selector",
    "semantics",
    "sep",
    "separator",
    "separators",
    "serif",
    "set",
    "setdiff",
    "shape",
    "shape-rendering",
    "show",
    "sin",
    "sinh",
    "size",
    "sizes",
    "slope",
    "small",
    "solidcolor",
    "space",
    "spacer",
    "spacing",
    "span",
    "specification",
    "specularconstant",
    "specularConstant",
    "specularexponent",
    "specularExponent",
    "speed",
    "spreadmethod",
    "spreadMethod",
    "src",
    "srcdoc",
    "standby",
    "start",
    "startoffset",
    "startOffset",
    "stddeviation",
    "stdDeviation",
    "stemh",
    "stemv",
    "step",
    "stitchtiles",
    "stitchTiles",
    "stop",
    "stop-color",
    "stop-opacity",
    "stretchy",
    "strike",
    "strikethrough-position",
    "strikethrough-thickness",
    "string",
    "stroke",
    "stroke-dasharray",
    "stroke-dashoffset",
    "stroke-linecap",
    "stroke-linejoin",
    "stroke-miterlimit",
    "stroke-opacity",
    "stroke-width",
    "strong",
    "sub",
    "submit",
    "subscriptshift",
    "subset",
    "sum",
    "summary",
    "sup",
    "superscriptshift",
    "surfacescale",
    "surfaceScale",
    "switch",
    "symbol",
    "symmetric",
    "systemlanguage",
    "systemLanguage",
    "tabindex",
    "table-layout",
    "tablevalues",
    "tableValues",
    "tan",
    "tanh",
    "target",
    "targetx",
    "targetX",
    "targety",
    "targetY",
    "tbreak",
    "tel",
    "tendsto",
    "text",
    "text-align",
    "text-anchor",
    "text-decoration",
    "textlength",
    "textLength",
    "text-orientation",
    "textpath",
    "textPath",
    "text-rendering",
    "thickmathspace",
    "thinmathspace",
    "time",
    "times",
    "to",
    "top",
    "transform",
    "transition-delay",
    "transition-duration",
    "transition-property",
    "transitions",
    "transition-timing-function",
    "transpose",
    "tref",
    "true",
    "tspan",
    "tt",
    "type",
    "u",
    "u1",
    "u2",
    "ul",
    "underline-position",
    "underline-thickness",
    "unicode",
    "unicode-bidi",
    "unicode-range",
    "union",
    "units-per-em",
    "unselectable",
    "uplimit",
    "url",
    "use",
    "usemap",
    "valign",
    "v-alphabetic",
    "value",
    "values",
    "valuetype",
    "var",
    "variance",
    "vector",
    "vectorproduct",
    "version",
    "vert-adv-y",
    "vertical-align",
    "vert-origin-x",
    "vert-origin-y",
    "verythickmathspace",
    "verythinmathspace",
    "veryverythickmathspace",
    "veryverythinmathspace",
    "v-hanging",
    "video",
    "v-ideographic",
    "view",
    "viewbox",
    "viewBox",
    "viewtarget",
    "viewTarget",
    "visibility",
    "vkern",
    "vlink",
    "v-mathematical",
    "vspace",
    "wbr",
    "week",
    "when",
    "white-space",
    "width",
    "widths",
    "word-spacing",
    "word-wrap",
    "wrap",
    "writing-mode",
    "x",
    "x1",
    "x2",
    "xchannelselector",
    "xChannelSelector",
    "x-height",
    "xlink",
    "xlink:actuate",
    "xlink:arcrole",
    "xlink:href",
    "xlink:role",
    "xlink:show",
    "xlink:title",
    "xlink:type",
    "xml:base",
    "xml:lang",
    "xmlns",
    "xmlns:xlink",
    "xml:space",
    "xor",
    "xref",
    "y",
    "y1",
    "y2",
    "ychannelselector",
    "yChannelSelector",
    "z",
    "zoomandpan",
    "zoomAndPan",
];
