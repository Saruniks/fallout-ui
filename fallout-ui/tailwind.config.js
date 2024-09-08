module.exports = {
  content: [
    "./src/**/*.rs",
    "./index.html",
    "./src/**/*.html",
    "./src/**/*.css",
    "../fallout-ui/src/**/*.rs",
    "../fallout-ui/src/**/*.html",
    "../fallout-ui/src/**/*.css",
  ],
  theme: {
    extend: {
      borderWidth: {
        1: "1px",
      },
      zIndex: {
        popover: "10",
        nav: "20",
        "modal-bg": "30",
        modal: "31",
      },
      fontFamily: {
        sans: [
          "DM Sans",
          "ui-sans-serif",
          "system-ui",
          "-apple-system",
          "BlinkMacSystemFont",
          '"Segoe UI"',
          "Roboto",
          '"Helvetica Neue"',
          "Arial",
          '"Noto Sans"',
          "sans-serif",
          '"Apple Color Emoji"',
          '"Segoe UI Emoji"',
          '"Segoe UI Symbol"',
          '"Noto Color Emoji"',
        ],
        mono: [
          "DM Mono",
          "ui-monospace",
          "SFMono-Regular",
          "Menlo",
          "Monaco",
          "Consolas",
          '"Liberation Mono"',
          '"Courier New"',
          "monospace",
        ],
      },
      spacing: {
        "input-sm": "13.5rem",
        "input-md": "28rem",
      },
      maxWidth: {
        "input-sm": "13.5rem",
        "input-md": "28rem",
      },
    },
  },
  variants: {},
  plugins: [
    require('@tailwindcss/typography'),
  ],
};
