const { fontFamily } = require('tailwindcss/defaultTheme');

/** @type {import('tailwindcss').Config} */
module.exports = {
    content: ['./templates/*.html'],
    theme: {
        extend: {
      colors: {
				theme: {
					light: "#f2f2e6",
					dark: "#b5aa79"
				},
				lemonSugar: {
					50: "#f3f5e8",
					100: "#f5f7ea",
					200: "#f8fbed",
					300: "#fafdee",
					400: "#f8fde9",
					500: "#f3f9e0",
					600: "#eaf1d1",
					700: "#dee5bd",
					800: "#d0d6a6",
					900: "#c2c68f",
				},
				mintCondition: {
					50: "#e8f5e8",
					100: "#eaf7ec",
					200: "#edfbf3",
					300: "#eefdf7",
					400: "#e9fdf7",
					500: "#e0f9f2",
					600: "#d1f1e7",
					700: "#bde5d2",
					800: "#a6d6b6",
					900: "#8fc692",
				},
				hostaFlower: {
					50: "#f2e8ef",
					100: "#f2e9f1",
					200: "#efecf1",
					300: "#ecebef",
					400: "#e7e7ec",
					500: "#dddde7",
					600: "#cfcee0",
					700: "#c2bbd7",
					800: "#bba5cc",
					900: "#bf8fc1",
				},
				bridesBlush: {
					50: "#f5efe8",
					100: "#f7f0ea",
					200: "#fbf2ed",
					300: "#fdf1ee",
					400: "#fdece9",
					500: "#f9e3e0",
					600: "#f1d5d1",
					700: "#e5c4bd",
					800: "#d6b2a6",
					900: "#c6a28f",
				},
			},
            fontFamily: {
                sans: ['Inter var', ...fontFamily.sans],
            },
        },
    },
  plugins: [require('@tailwindcss/typography')]
};
