<script>
  import { tick } from "svelte";
  import rawMap from "$lib/assets/world-map.svg?raw";
  import { app } from "$lib/stores/app.svelte.js";

  const svg = (() => {
    let s = rawMap
      .replace(/<\?xml[\s\S]*?\?>/, "")
      .replace(/<!DOCTYPE[\s\S]*?>/, "")
      .replace(/<title>[\s\S]*?<\/title>/, "")
      .replace(/<desc>[\s\S]*?<\/desc>/, "")
      .replace('width="784.077px"', 'width="100%"')
      .replace('height="458.627px"', 'height="100%"');
    const defs =
      '<defs><pattern id="rika-dots" width="5.2" height="5.2" patternUnits="userSpaceOnUse"><circle cx="1.1" cy="1.1" r="1" fill="currentColor"/></pattern></defs>';
    s = s.replace("<g>", defs + '<g fill="url(#rika-dots)">');
    return s;
  })();

  let container = $state(null);
  let marker = $state({ x: 26, y: 34 });

  async function place() {
    await tick();
    if (!container) return;
    const code = app.server?.code;
    const path = code ? container.querySelector("#" + CSS.escape(code)) : null;
    const svgEl = container.querySelector("svg");
    if (path && svgEl && svgEl.viewBox && svgEl.viewBox.baseVal.width) {
      const bb = path.getBBox();
      const vb = svgEl.viewBox.baseVal;
      marker = {
        x: ((bb.x + bb.width / 2 - vb.x) / vb.width) * 100,
        y: ((bb.y + bb.height / 2 - vb.y) / vb.height) * 100,
      };
    }
  }

  $effect(() => {
    const code = app.server?.code;
    if (code) place();
  });
</script>

<div bind:this={container} class="relative w-full text-ink/25 dark:text-white/20">
  {@html svg}
  <span
    class="pointer-events-none absolute -translate-x-1/2 -translate-y-1/2 transition-all duration-700 ease-out"
    style="left:{marker.x}%; top:{marker.y}%;"
  >
    <span
      class="absolute inline-flex h-4 w-4 rounded-full bg-brand/40"
      style="animation: rika-pulse 2.4s ease-out infinite;"
    ></span>
    <span
      class="absolute inline-flex h-4 w-4 rounded-full bg-brand/40"
      style="animation: rika-pulse 2.4s ease-out infinite 1.2s;"
    ></span>
    <span
      class="relative grid h-4 w-4 place-items-center rounded-full bg-brand ring-4 ring-brand/25"
    >
      <span class="h-1.5 w-1.5 rounded-full bg-white"></span>
    </span>
  </span>
</div>
