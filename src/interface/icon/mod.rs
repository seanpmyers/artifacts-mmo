pub type SvgIcon = &'static str;

pub const HOME_SVG: SvgIcon = r#"
<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor"
		stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="feather feather-home">
		<path d="M3 9l9-7 9 7v11a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2z"></path>
		<polyline points="9 22 9 12 15 12 15 22"></polyline>
	</svg>
"#;

pub const ARROWS_CLOCKWISE: SvgIcon = r#"
<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 256 256" fill="none" stroke="currentColor">
	<rect width="256" height="256" fill="none" />
	<polyline points="168 96 216 96 216 48" fill="none" stroke="currentColor" stroke-linecap="round"
		stroke-linejoin="round" stroke-width="16" />
	<path d="M216,96,187.72,67.72A88,88,0,0,0,64,67" fill="none" stroke="currentColor"
		stroke-linecap="round" stroke-linejoin="round" stroke-width="16" />
	<polyline points="88 160 40 160 40 208" fill="none" stroke="currentColor" stroke-linecap="round"
		stroke-linejoin="round" stroke-width="16" />
	<path d="M40,160l28.28,28.28A88,88,0,0,0,192,189" fill="none" stroke="currentColor"
		stroke-linecap="round" stroke-linejoin="round" stroke-width="16" />
</svg>
"#;

pub const CLOUD_ARROW_DOWNLOAD: SvgIcon = r#"
<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor"
	stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="feather feather-download-cloud">
	<polyline points="8 17 12 21 16 17"></polyline>
	<line x1="12" y1="12" x2="12" y2="21"></line>
	<path d="M20.88 18.09A5 5 0 0 0 18 9h-1.26A8 8 0 1 0 3 16.29"></path>
</svg>
"#;

pub const THEME_TOGGLE_SVG: SvgIcon = r#"
<svg xmlns="http://www.w3.org/2000/svg" width="1em" height="1em" viewBox="0 0 24 24" fill="none" stroke="currentColor">
	<path fill="currentColor"
		d="M7.5 2c-1.79 1.15-3 3.18-3 5.5s1.21 4.35 3.03 5.5C4.46 13 2 10.54 2 7.5A5.5 5.5 0 0 1 7.5 2m11.57 1.5l1.43 1.43L4.93 20.5L3.5 19.07zm-6.18 2.43L11.41 5L9.97 6l.42-1.7L9 3.24l1.75-.12l.58-1.65L12 3.1l1.73.03l-1.35 1.13zm-3.3 3.61l-1.16-.73l-1.12.78l.34-1.32l-1.09-.83l1.36-.09l.45-1.29l.51 1.27l1.36.03l-1.05.87zM19 13.5a5.5 5.5 0 0 1-5.5 5.5c-1.22 0-2.35-.4-3.26-1.07l7.69-7.69c.67.91 1.07 2.04 1.07 3.26m-4.4 6.58l2.77-1.15l-.24 3.35zm4.33-2.7l1.15-2.77l2.2 2.54zm1.15-4.96l-1.14-2.78l3.34.24zM9.63 18.93l2.77 1.15l-2.53 2.19z" />
</svg>
"#;
