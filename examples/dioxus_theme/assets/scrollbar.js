// Progressive enhancement: keep the native scrollbar until this element is ready.
if (!customElements.get('overlay-scrollbar')) {
  customElements.define('overlay-scrollbar', class extends HTMLElement {
    connectedCallback() {
      this.thumb = document.createElement('span');
      this.append(this.thumb);
      this.tabIndex = 0;
      this.setAttribute('role', 'scrollbar');
      this.setAttribute('aria-label', 'Page scroll');
      this.setAttribute('aria-orientation', 'vertical');
      this.setAttribute('aria-controls', 'workbench-content');
      this.setAttribute('aria-valuemin', '0');
      this.events = new AbortController();
      const options = { signal: this.events.signal };
      this.update = () => {
        const root = document.scrollingElement;
        const height = innerHeight;
        this.max = Math.max(0, root.scrollHeight - height);
        this.classList.toggle('no-overflow', this.max === 0);
        this.tabIndex = this.max ? 0 : -1;
        this.setAttribute('aria-hidden', String(this.max === 0));
        const track = this.clientHeight;
        const thumbHeight = Math.min(track, Math.max(36, track * height / root.scrollHeight));
        this.travel = track - thumbHeight;
        this.thumb.style.height = `${thumbHeight}px`;
        this.thumb.style.transform = `translateY(${this.max ? root.scrollTop / this.max * this.travel : 0}px)`;
        this.setAttribute('aria-valuemax', String(this.max));
        this.setAttribute('aria-valuenow', String(Math.round(root.scrollTop)));
      };
      window.addEventListener('scroll', this.update, { ...options, passive: true });
      window.addEventListener('resize', this.update, options);
      this.addEventListener('pointerdown', event => {
        if (event.button !== 0 || !this.max) return;
        event.preventDefault();
        this.focus({ preventScroll: true });
        this.setPointerCapture(event.pointerId);
        this.classList.add('dragging');
        const thumbRect = this.thumb.getBoundingClientRect();
        this.grabOffset = event.target === this.thumb ? event.clientY - thumbRect.top : thumbRect.height / 2;
        this.move(event);
      }, options);
      this.move = event => {
        if (!this.hasPointerCapture(event.pointerId) || !this.travel) return;
        const fraction = (event.clientY - this.getBoundingClientRect().top - this.grabOffset) / this.travel;
        window.scrollTo({ top: Math.max(0, Math.min(1, fraction)) * this.max, behavior: 'instant' });
      };
      this.addEventListener('pointermove', this.move, options);
      this.addEventListener('pointerup', event => {
        if (this.hasPointerCapture(event.pointerId)) this.releasePointerCapture(event.pointerId);
      }, options);
      this.addEventListener('lostpointercapture', () => this.classList.remove('dragging'), options);
      this.addEventListener('keydown', event => {
        const current = document.scrollingElement.scrollTop;
        const positions = { ArrowDown: current + 40, ArrowUp: current - 40,
          PageDown: current + innerHeight * .9, PageUp: current - innerHeight * .9,
          Home: 0, End: this.max, ' ': current + innerHeight * (event.shiftKey ? -.9 : .9) };
        if (!(event.key in positions)) return;
        event.preventDefault();
        window.scrollTo({ top: positions[event.key], behavior: 'instant' });
      }, options);
      this.observer = new ResizeObserver(this.update);
      this.observer.observe(document.body);
      this.setAttribute('data-ready', '');
      this.update();
    }
    disconnectedCallback() {
      this.events.abort();
      this.observer.disconnect();
      this.thumb.remove();
    }
  });
}
