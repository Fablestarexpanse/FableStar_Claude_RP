import type { ViewTransform, VisibleChunks } from './types';

export class TerrainViewport {
  private transform: ViewTransform;
  private canvas: HTMLCanvasElement;
  private isDragging = false;
  private lastMouseX = 0;
  private lastMouseY = 0;
  private worldWidth = 1536;  // Default world dimensions
  private worldHeight = 768;
  private onTransformChange: (transform: ViewTransform) => void;
  private onVisibleChunksChange: (chunks: VisibleChunks) => void;

  constructor(
    canvas: HTMLCanvasElement,
    onTransformChange: (transform: ViewTransform) => void,
    onVisibleChunksChange: (chunks: VisibleChunks) => void
  ) {
    this.canvas = canvas;
    this.onTransformChange = onTransformChange;
    this.onVisibleChunksChange = onVisibleChunksChange;
    
    this.transform = {
      translateX: 0,
      translateY: 0,
      scale: 1.0,
      rotation: 0
    };

    this.setupEventListeners();
  }

  private setupEventListeners() {
    // Mouse wheel for zoom
    this.canvas.addEventListener('wheel', this.handleWheel.bind(this), { passive: false });

    // Mouse drag for pan
    this.canvas.addEventListener('mousedown', this.handleMouseDown.bind(this));
    this.canvas.addEventListener('mousemove', this.handleMouseMove.bind(this));
    this.canvas.addEventListener('mouseup', this.handleMouseUp.bind(this));
    this.canvas.addEventListener('mouseleave', this.handleMouseUp.bind(this));

    // Touch events for mobile
    this.canvas.addEventListener('touchstart', this.handleTouchStart.bind(this), { passive: false });
    this.canvas.addEventListener('touchmove', this.handleTouchMove.bind(this), { passive: false });
    this.canvas.addEventListener('touchend', this.handleTouchEnd.bind(this));
  }

  private handleWheel(e: WheelEvent) {
    e.preventDefault();

    const rect = this.canvas.getBoundingClientRect();
    const mouseX = e.clientX - rect.left;
    const mouseY = e.clientY - rect.top;

    // Calculate zoom limits
    // Temporarily allow both directions so user can test
    const minScale = 0.1;   // Allow zooming OUT
    const maxScale = 20.0;  // Allow zooming IN

    // Zoom centered on cursor
    // deltaY < 0 = scroll up/toward you = zoom IN (see more detail)
    // deltaY > 0 = scroll down/away from you = zoom OUT (see less detail)
    const zoomFactor = e.deltaY < 0 ? 1.1 : 0.9;
    const newScale = Math.max(minScale, Math.min(maxScale, this.transform.scale * zoomFactor));

    // Adjust translation to keep cursor position fixed
    const scaleRatio = newScale / this.transform.scale;
    this.transform.translateX = mouseX - (mouseX - this.transform.translateX) * scaleRatio;
    this.transform.translateY = mouseY - (mouseY - this.transform.translateY) * scaleRatio;
    this.transform.scale = newScale;

    this.notifyTransformChange();
  }

  private handleMouseDown(e: MouseEvent) {
    if (e.button === 0) { // Left button
      this.isDragging = true;
      this.lastMouseX = e.clientX;
      this.lastMouseY = e.clientY;
      this.canvas.style.cursor = 'grabbing';
    }
  }

  private handleMouseMove(e: MouseEvent) {
    if (this.isDragging) {
      const dx = e.clientX - this.lastMouseX;
      const dy = e.clientY - this.lastMouseY;

      this.transform.translateX += dx;
      this.transform.translateY += dy;

      this.lastMouseX = e.clientX;
      this.lastMouseY = e.clientY;

      this.notifyTransformChange();
    }
  }

  private handleMouseUp() {
    this.isDragging = false;
    this.canvas.style.cursor = 'grab';
  }

  private handleTouchStart(e: TouchEvent) {
    e.preventDefault();
    if (e.touches.length === 1) {
      this.isDragging = true;
      this.lastMouseX = e.touches[0].clientX;
      this.lastMouseY = e.touches[0].clientY;
    }
  }

  private handleTouchMove(e: TouchEvent) {
    e.preventDefault();
    if (this.isDragging && e.touches.length === 1) {
      const dx = e.touches[0].clientX - this.lastMouseX;
      const dy = e.touches[0].clientY - this.lastMouseY;

      this.transform.translateX += dx;
      this.transform.translateY += dy;

      this.lastMouseX = e.touches[0].clientX;
      this.lastMouseY = e.touches[0].clientY;

      this.notifyTransformChange();
    }
  }

  private handleTouchEnd() {
    this.isDragging = false;
  }

  private notifyTransformChange() {
    this.onTransformChange(this.transform);
    this.updateVisibleChunks();
  }

  private updateVisibleChunks() {
    // Calculate visible area in world coordinates
    const canvasWidth = this.canvas.width;
    const canvasHeight = this.canvas.height;

    // Transform corners to world space
    const corners = [
      this.screenToWorld(0, 0),
      this.screenToWorld(canvasWidth, 0),
      this.screenToWorld(0, canvasHeight),
      this.screenToWorld(canvasWidth, canvasHeight)
    ];

    // Find bounds
    let minX = Infinity, maxX = -Infinity;
    let minZ = Infinity, maxZ = -Infinity;

    for (const [x, z] of corners) {
      minX = Math.min(minX, x);
      maxX = Math.max(maxX, x);
      minZ = Math.min(minZ, z);
      maxZ = Math.max(maxZ, z);
    }

    // Convert to chunk coordinates (assuming 128-cell chunks)
    const chunkSize = 128;
    const minChunkX = Math.floor(minX / chunkSize);
    const maxChunkX = Math.ceil(maxX / chunkSize);
    const minChunkZ = Math.floor(minZ / chunkSize);
    const maxChunkZ = Math.ceil(maxZ / chunkSize);

    // Build set of visible chunks
    const chunks = new Set<string>();
    for (let cz = minChunkZ; cz <= maxChunkZ; cz++) {
      for (let cx = minChunkX; cx <= maxChunkX; cx++) {
        chunks.add(`${cx},${cz}`);
      }
    }

    this.onVisibleChunksChange({
      chunks,
      minX: minChunkX,
      maxX: maxChunkX,
      minZ: minChunkZ,
      maxZ: maxChunkZ
    });
  }

  private screenToWorld(screenX: number, screenY: number): [number, number] {
    // Inverse transform from screen to world coordinates
    const x = (screenX - this.transform.translateX) / this.transform.scale;
    const z = (screenY - this.transform.translateY) / this.transform.scale;
    return [x, z];
  }

  public worldToScreen(worldX: number, worldZ: number): [number, number] {
    const x = worldX * this.transform.scale + this.transform.translateX;
    const y = worldZ * this.transform.scale + this.transform.translateY;
    return [x, y];
  }

  public getTransform(): ViewTransform {
    return { ...this.transform };
  }

  public setTransform(transform: Partial<ViewTransform>) {
    this.transform = { ...this.transform, ...transform };
    this.notifyTransformChange();
  }

  public setWorldDimensions(width: number, height: number) {
    this.worldWidth = width;
    this.worldHeight = height;
  }

  public resetView() {
    this.transform = {
      translateX: 0,
      translateY: 0,
      scale: 1.0,
      rotation: 0
    };
    this.notifyTransformChange();
  }

  public getViewMatrix(): number[] {
    // Return 3x3 affine transform matrix for WebGL
    const { translateX, translateY, scale, rotation } = this.transform;
    const cos = Math.cos(rotation);
    const sin = Math.sin(rotation);

    return [
      scale * cos, scale * sin, 0,
      -scale * sin, scale * cos, 0,
      translateX, translateY, 1
    ];
  }

  public destroy() {
    // Clean up event listeners
    this.canvas.removeEventListener('wheel', this.handleWheel.bind(this));
    this.canvas.removeEventListener('mousedown', this.handleMouseDown.bind(this));
    this.canvas.removeEventListener('mousemove', this.handleMouseMove.bind(this));
    this.canvas.removeEventListener('mouseup', this.handleMouseUp.bind(this));
    this.canvas.removeEventListener('mouseleave', this.handleMouseUp.bind(this));
  }
}
