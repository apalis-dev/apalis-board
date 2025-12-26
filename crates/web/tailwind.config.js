/** @type {import('tailwindcss').Config} */
const colors = require("tailwindcss/colors");


const charcoal = {
  100: "#E8E9EC",
  200: "#D7D9DD",
  300: "#B5B8C0",
  400: "#878C99",
  500: "#5F6570",
  550: "#4D525B",
  600: "#3B3E45",
  650: "#2C3034",
  700: "#272A2E",
  750: "#212327",
  775: "#1C1E21",
  800: "#1A1B1F",
  850: "#15171A",
  900: "#121317",
  950: "#0D0E12",
  1000: "#0B0C0F",
};

const mint = {
  50: "#F0FDF4",
  100: "#DDFBE6",
  200: "#BDF5D0",
  300: "#87EBA9",
  400: "#4FD97E",
  500: "#28BF5C",
  600: "#1B9E48",
  700: "#197C3C",
  800: "#196233",
  900: "#16512C",
  950: "#062D15",
};

const lavender = {
  50: "##f4f2ff",
  100: "#eae8ff",
  200: "#d7d4ff",
  300: "#bab2ff",
  400: "#826dff",
  500: "#7655fd",
  600: "#6532f5",
  700: "#5620e1",
  800: "#481abd",
  900: "#3d189a",
  950: "#230c69",
};

/** Text colors */
const primary = charcoal[500];
const secondary = charcoal[650];
const tertiary = charcoal[700];
const textLink = lavender[400];
const textDimmed = charcoal[400];
const textBright = charcoal[200];
const backgroundBright = charcoal[900];
const backgroundDimmed = charcoal[850];
const gridBright = charcoal[700];
const gridDimmed = charcoal[750];
const success = mint[500];
const pending = colors.blue[500];
const warning = colors.amber[500];
const error = colors.rose[600];

/** Other variables */
const radius = "0.5rem";

/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["./src/**/*.rs", "./index.html"],
  theme: {
    container: {
      center: true, 
      padding: "2rem",
      screens: {
        "2xl": "1400px",
      },
    },
    extend: {
      fontFamily: {
        sans: ["Geist Variable", "Helvetica Neue", "Helvetica", "Arial", "sans-serif"],
        mono: ["Geist Mono Variable", "monaco", "Consolas", "Lucida Console", "monospace"],
      },
      fontSize: {
        xxs: [
          "0.65rem",
          {
            lineHeight: "0.75rem",
            letterSpacing: "-0.01em",
            fontWeight: "500",
          },
        ],
        "2sm": [
          "0.8125rem",
          {
            lineHeight: "0.875rem",
            letterSpacing: "-0.01em",
            fontWeight: "500",
          },
        ],
      },
      colors: {
        charcoal,
        lavender,
        mint,
        primary,
        secondary,
        tertiary,
        "text-link": textLink,
        "text-dimmed": textDimmed,
        "text-bright": textBright,
        "background-bright": backgroundBright,
        "background-dimmed": backgroundDimmed,
        "grid-bright": gridBright,
        "grid-dimmed": gridDimmed,
        success,
        pending,
        warning,
        error,
      },
      focusStyles: {
        outline: "1px solid",
        outlineOffset: "0px",
        outlineColor: textLink,
        borderRadius: "3px",
      },
      borderRadius: {
        lg: radius,
        md: `calc(${radius} - 2px)`,
        sm: `calc(${radius} - 4px)`,
      },
      outlineWidth: {
        3: "3px",
      },
      textShadow: {
        custom: "1px 1px 1px rgba(0, 0, 0, 0.5)", // Offset-X | Offset-Y | Blur radius | Color
      },
      keyframes: {
        "accordion-down": {
          from: { height: 0 },
          to: { height: "var(--radix-accordion-content-height)" },
        },
        "accordion-up": {
          from: { height: "var(--radix-accordion-content-height)" },
          to: { height: 0 },
        },
        float: {
          "0%": { transform: "translatey(0px)" },
          "50%": { transform: "translatey(7px)" },
          "100%": { transform: "translatey(0px)" },
        },
        "tile-move": {
          "0%": { "background-position": "0px" },
          "100%": { "background-position": "8px" },
        },
        "tile-move-offset": {
          "0%": { "background-position": "-1px" },
          "100%": { "background-position": "7px" },
        },
      },
      animation: {
        "accordion-down": "accordion-down 0.2s ease-out",
        "accordion-up": "accordion-up 0.2s ease-out",
        "tile-scroll": "tile-move 0.5s infinite linear",
        "tile-scroll-offset": "tile-move-offset 0.5s infinite linear",
      },
      backgroundImage: {
        "gradient-radial": "radial-gradient(closest-side, var(--tw-gradient-stops))",
        "gradient-primary": `linear-gradient(90deg, acid-500 0%, toxic-500 100%)`,
        "gradient-primary-hover": `linear-gradient(80deg, acid-600 0%, toxic-600 100%)`,
        "gradient-secondary": `linear-gradient(90deg, hsl(271 91 65) 0%, hsl(221 83 53) 100%)`,
        "gradient-radial-secondary ": `radial-gradient(hsl(271 91 65), hsl(221 83 53))`,
      },
      gridTemplateColumns: {
        carousel: "repeat(6, 200px)",
      },
      screens: {
        "lg-height": { raw: "(max-height: 750px)" },
        "md-height": { raw: "(max-height: 600px)" },
      },
      width: {
        0.75: "0.1875rem",
        4.5: "1.125rem",
      },
      height: {
        0.75: "0.1875rem",
        4.5: "1.125rem",
      },
      size: {
        4.5: "1.125rem",
      },
    },
  },
  plugins: [
    require("@tailwindcss/container-queries"),
    require("@tailwindcss/forms"),
    require("@tailwindcss/typography"),
    require("tailwindcss-animate"),
    require("tailwind-scrollbar"),
    require("tailwind-scrollbar-hide"),
    require("tailwindcss-textshadow"),
    function ({ addUtilities, theme }) {
      const focusStyles = theme("focusStyles", {});
      addUtilities({
        ".focus-custom": {
          "&:focus-visible": focusStyles,
        },
      });
    },
  ],
};
