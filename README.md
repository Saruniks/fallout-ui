# fallout-ui

Reference component library made for `Yew.rs`, using only `tailwind` and `notifier.js`.

Features:

- A custom form state management solution called `FormFields` similar to `useForm` in react ecosystem
- Colors, fonts and other configuration available via `./fallout-ui/tailwind.config.js`
- Components:
  - Modal
  - Table
  - Accordion table
  - Various input components that go well with the `FormFields` macro
  - Button
  - Link
  - Callout
  - Expandable list
  - Toast
  - Vertical data list
- Few utility hooks

## Storybook

https://fallout-ui.link

## Running this project

Install trunk:

```
cargo install trunk
```

Go to the demo app:

```
cd ./demo-app
```

Serve the app:

```
trunk serve
```
