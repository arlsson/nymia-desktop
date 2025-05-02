# Enabling Window Dragging in Tauri 2.0 + Svelte with `titleBarStyle: "Overlay"`

When using `"titleBarStyle": "Overlay"` in a macOS Tauri 2.0 app, the native draggable area is removed. To restore this functionality, follow these steps:

## 1. Add a Draggable Region in Your Svelte Component

In your Svelte file, add a `div` at the top of your UI with the `data-tauri-drag-region` attribute:

```html
<div class="titlebar" data-tauri-drag-region>
  <!-- Custom title bar content like buttons or app name -->
</div>
```

**Note:** Any interactive elements inside this bar (e.g., buttons) should not have the `data-tauri-drag-region` attribute to avoid drag conflicts.

## 2. Style the Draggable Region with CSS

Use CSS to define the appearance and behavior of the draggable title bar:

```css
.titlebar {
  height: 30px;
  background-color: #2e2e2e;
  user-select: none;
  display: flex;
  align-items: center;
  padding: 0 10px;
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  z-index: 1000;
}
```

## 3. Configure Window Permissions in `tauri.conf.json`

Update your `tauri.conf.json` file to allow window dragging:

```json
"security": {
  "capabilities": [
    {
      "identifier": "draggable-window",
      "windows": ["*"],
      "permissions": ["core:window:allow-start-dragging"]
    }
  ]
}
```

Place this inside the `tauri > allowlist` or top-level `security` object depending on your version structure.

## 4. Set `titleBarStyle` to `"Overlay"`

Ensure the Tauri window is configured to use the overlay style in `tauri.conf.json`:

```json
"windows": [
  {
    "label": "main",
    "title": "My App",
    "width": 800,
    "height": 600,
    "titleBarStyle": "Overlay"
  }
]
```

---

With these steps completed, your app should support window dragging using a custom title bar on macOS.
