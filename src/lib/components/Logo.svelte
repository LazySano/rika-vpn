<script>
  let { size = 30, withText = true } = $props();
</script>

<div class="flex items-center gap-2.5">
  <svg width={size} height={size} viewBox="0 0 40 40" fill="none" aria-hidden="true">
    <defs>
      <linearGradient id="rikaLogoGrad" x1="0" y1="0" x2="40" y2="40" gradientUnits="userSpaceOnUse">
        <stop stop-color="#8b7bff" />
        <stop offset="1" stop-color="#5a49d6" />
      </linearGradient>
    </defs>
    <path d="M20 2.5 34.5 10.8v18.4L20 37.5 5.5 29.2V10.8z" fill="url(#rikaLogoGrad)" />
    <path
      d="M20 11.4l-5.4 3.1v6.1c0 3.9 2.4 7 5.4 8 3-1 5.4-4.1 5.4-8v-6.1z"
      fill="#ffffff"
      fill-opacity="0.96"
    />
    <circle cx="20" cy="20.3" r="1.9" fill="#5a49d6" />
  </svg>
  {#if withText}
    <span class="text-lg font-extrabold tracking-tight text-ink">
      Rika<span class="text-brand">VPN</span>
    </span>
  {/if}
</div>
