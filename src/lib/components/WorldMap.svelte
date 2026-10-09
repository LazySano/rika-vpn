<script>
  import rawMap from "$lib/assets/world-map.svg?raw";

  const svg = (() => {
    let s = rawMap
      .replace(/<\?xml[\s\S]*?\?>/, "")
      .replace(/<!DOCTYPE[\s\S]*?>/, "")
      .replace('width="784.077px"', 'width="100%"')
      .replace('height="458.627px"', 'height="100%"');
    const defs =
      '<defs><pattern id="rika-dots" width="5.2" height="5.2" patternUnits="userSpaceOnUse"><circle cx="1.1" cy="1.1" r="1" fill="currentColor"/></pattern></defs>';
    s = s.replace("<g>", defs + '<g fill="url(#rika-dots)">');
    return s;
  })();
</script>

<div class="relative w-full text-ink/25 dark:text-white/20">
  {@html svg}
  <span
    class="pointer-events-none absolute left-[19%] top-[40%] -translate-x-1/2 -translate-y-1/2"
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
