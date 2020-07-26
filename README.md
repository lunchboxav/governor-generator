# Governor Generator

This project aims to generate a React component from component designed and defined in Figma. Initial aim is to create atom-level component, such as button, text fields, so we can create vieew-level form. 

The main goal is to be able to create React component along with the required CSS styling straight from Figma. Adding logic to the component is outside of the scope, at least for the time being. 

By creating this, I hope to improve designer-developer workflow, so the designer can provide React component that the developer can augment with the required logic.

## PoC

A component created in Figma, can be accessed by its API, and it'll return JSON that describes the component's visual description. Full file can be seen in `reference.json`. Afterwards, we need to extract these values to fully populate the CSS property, such as this:

```
.btn {
  background-color: #6fcf97;
  border: 1px solid #21864b;
  padding: 8px 16px;
  color: #21864b;
  text-align: center;
  text-transform: uppercase;
  font-family: "Oswald", sans-serif;
  font-size: 12px;
}
```

### Notable Points

Translating from JSON to CSS, might not be straightforward, here are some of the points that I discovered

- Colors in JSON is described in RGBA, but with scale for 0-1 on each channel. This needs to be multiplied by 255 to get proper value for CSS.
- Border is described in `stroke` property in JSON
- Others, including padding, font-size, font-alignment and text-transform, can be grabbed directly from the JSON file
- Properties written in camel case should be transformed into dash-case so it can be used to create CSS property.

### Exporting the Component

At the moment, I'm thinking of creating both JS file that describes the component and the CSS file that describes the styling. The JS file will refer to this component. CSS module could be useful in this sense. I'm not thinking of using CSS-in-JS for the time being, I think CSS file can be more universal, and how it's being used will depend on each use case. 

So a component might be exported like this

- Component
  - Component.jsx
  - ComponentStyle.css
