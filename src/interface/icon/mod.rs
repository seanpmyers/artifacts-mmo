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

pub const MUTED_SVG: SvgIcon = r#"
<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 256 256">
	<rect width="256" height="256" fill="none" />
	<path d="M80,168H32a8,8,0,0,1-8-8V96a8,8,0,0,1,8-8H80l72-56V224Z" fill="none"
		stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="16" />
	<line x1="240" y1="104" x2="192" y2="152" fill="none" stroke="currentColor" stroke-linecap="round"
		stroke-linejoin="round" stroke-width="16" />
	<line x1="240" y1="152" x2="192" y2="104" fill="none" stroke="currentColor" stroke-linecap="round"
		stroke-linejoin="round" stroke-width="16" />
	<line x1="80" y1="88" x2="80" y2="168" fill="none" stroke="currentColor" stroke-linecap="round"
		stroke-linejoin="round" stroke-width="16" />
</svg>
"#;

pub const SOUND_ON_SVG: SvgIcon = r#"
<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 256 256">
	<rect width="256" height="256" fill="none" />
	<path d="M80,168H32a8,8,0,0,1-8-8V96a8,8,0,0,1,8-8H80l72-56V224Z" fill="none"
		stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="16" />
	<line x1="80" y1="88" x2="80" y2="168" fill="none" stroke="currentColor" stroke-linecap="round"
		stroke-linejoin="round" stroke-width="16" />
	<path d="M192,106.85a32,32,0,0,1,0,42.3" fill="none" stroke="currentColor" stroke-linecap="round"
		stroke-linejoin="round" stroke-width="16" />
	<path d="M221.67,80a72,72,0,0,1,0,96" fill="none" stroke="currentColor" stroke-linecap="round"
		stroke-linejoin="round" stroke-width="16" />
</svg>
"#;

pub const EYE_OPEN: SvgIcon = r#"
	<g fill='none'>
		<path fill='#9b9b9b' d='M16.184 1C7.814 1 1.028 7.786 1.028 16.156s6.785 15.157 15.156 15.157c8.37 0 15.156-6.786 15.156-15.157C31.34 7.786 24.555 1 16.184 1' />
		<path fill='#fff' d='M30.34 16.156c0 7.819-6.338 14.157-14.156 14.157S2.028 23.975 2.028 16.156C2.028 8.338 8.366 2 16.184 2S30.34 8.338 30.34 16.156' />
		<path fill='#7d4533' d='M24.997 15.984a8.984 8.984 0 1 1-17.97 0a8.984 8.984 0 0 1 17.97 0' />
		<path fill='#1c1c1c' d='M21.028 16a5 5 0 1 1-10 0a5 5 0 0 1 10 0' />
		<path fill='#fff' d='M15.028 13a2 2 0 1 1-4 0a2 2 0 0 1 4 0' />
	</g>
"#;

pub const EYE_CLOSED: SvgIcon = r#"
<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><path fill="currentColor" d="M2 5.27L3.28 4L20 20.72L18.73 22l-3.08-3.08c-1.15.38-2.37.58-3.65.58c-5 0-9.27-3.11-11-7.5c.69-1.76 1.79-3.31 3.19-4.54zM12 9a3 3 0 0 1 3 3a3 3 0 0 1-.17 1L11 9.17A3 3 0 0 1 12 9m0-4.5c5 0 9.27 3.11 11 7.5a11.8 11.8 0 0 1-4 5.19l-1.42-1.43A9.86 9.86 0 0 0 20.82 12A9.82 9.82 0 0 0 12 6.5c-1.09 0-2.16.18-3.16.5L7.3 5.47c1.44-.62 3.03-.97 4.7-.97M3.18 12A9.82 9.82 0 0 0 12 17.5c.69 0 1.37-.07 2-.21L11.72 15A3.064 3.064 0 0 1 9 12.28L5.6 8.87c-.99.85-1.82 1.91-2.42 3.13"/></svg>
"#;
