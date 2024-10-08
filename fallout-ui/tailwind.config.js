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
      colors: {
        black: "#231f20",
        white: "#ffffff",
        transparent: "transparent",

        primary: "#2C41FF",
        secondary: "#1A151B",
        thirdly: "#DBEAFE",
        danger: "#EA1D25",
        success: "#29A36E",

        "reacting-primary": "#0014C7",
        "reacting-secondary": "#1E1825",
        "reacting-danger": "#950E13",
        "reacting-success": "#145237",

        "paper-primary": "#D6DAFF",
        "paper-secondary": "#EAE7EF",
        "paper-danger": "#FCDADB",

        "paper-success": "#DEF7EC",
        "washed-out-primary": "#808CFF",
        "washed-out-secondary": "#9B8BAA",
        "washed-out-thirdly": "#C5D3EB",
        "washed-out-danger": "#F48A8E",
        "washed-out-success": "#99E6C4",
      },
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
