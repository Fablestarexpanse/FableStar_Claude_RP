export class BrushPreview {
  private canvas: HTMLCanvasElement;
  private ctx: CanvasRenderingContext2D;
  private isVisible = false;
  private x = 0;
  private y = 0;
  private radius = 10;

  constructor(canvas: HTMLCanvasElement) {
    this.canvas = canvas;
    const ctx = canvas.getContext('2d');
    if (!ctx) {
      throw new Error('Failed to get 2D context for brush preview');
    }
    this.ctx = ctx;
  }

  public show(x: number, y: number, radius: number) {
    this.isVisible = true;
    this.x = x;
    this.y = y;
    this.radius = radius;
    this.render();
  }

  public hide() {
    this.isVisible = false;
    this.clear();
  }

  public update(x: number, y: number) {
    if (this.isVisible) {
      this.x = x;
      this.y = y;
      this.render();
    }
  }

  public setRadius(radius: number) {
    this.radius = radius;
    if (this.isVisible) {
      this.render();
    }
  }

  private render() {
    this.clear();
    
    if (!this.isVisible) {
      return;
    }

    this.ctx.save();

    // Draw outer circle
    this.ctx.strokeStyle = 'rgba(255, 107, 53, 0.8)';
    this.ctx.lineWidth = 2;
    this.ctx.beginPath();
    this.ctx.arc(this.x, this.y, this.radius, 0, Math.PI * 2);
    this.ctx.stroke();

    // Draw inner circle
    this.ctx.strokeStyle = 'rgba(255, 107, 53, 0.4)';
    this.ctx.lineWidth = 1;
    this.ctx.beginPath();
    this.ctx.arc(this.x, this.y, this.radius * 0.5, 0, Math.PI * 2);
    this.ctx.stroke();

    // Draw crosshair
    this.ctx.strokeStyle = 'rgba(255, 107, 53, 0.6)';
    this.ctx.lineWidth = 1;
    this.ctx.beginPath();
    this.ctx.moveTo(this.x - 5, this.y);
    this.ctx.lineTo(this.x + 5, this.y);
    this.ctx.moveTo(this.x, this.y - 5);
    this.ctx.lineTo(this.x, this.y + 5);
    this.ctx.stroke();

    this.ctx.restore();
  }

  private clear() {
    this.ctx.clearRect(0, 0, this.canvas.width, this.canvas.height);
  }

  public resize(width: number, height: number) {
    this.canvas.width = width;
    this.canvas.height = height;
    if (this.isVisible) {
      this.render();
    }
  }
}
