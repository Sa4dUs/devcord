
export interface BubbleData {
  id: number;
  isColliding: boolean;
  isDragging: boolean;
  x: number;
  y: number;
}

export class BubbleContainerLogic {
  readonly bubbleSize = 100;
  readonly maxIterations = 5;
  readonly pushBackDistance = -5;

  constructor(private boundaryWidth: number = 400, private boundaryHeight: number = 400) {}

  setBoundary(width: number, height: number) {
    this.boundaryWidth = width;
    this.boundaryHeight = height;
  }

  clampPosition(x: number, y: number) {
    const maxX = this.boundaryWidth - this.bubbleSize;
    const maxY = this.boundaryHeight - this.bubbleSize;

    return {
      x: Math.min(Math.max(x, 0), maxX),
      y: Math.min(Math.max(y, 0), maxY),
    };
  }

  isColliding(a: DOMRect, b: DOMRect): boolean {
    return !(
      a.right < b.left ||
      a.left > b.right ||
      a.bottom < b.top ||
      a.top > b.bottom
    );
  }

  generateRect(x: number, y: number): DOMRect {
    return {
      x,
      y,
      width: this.bubbleSize,
      height: this.bubbleSize,
      left: x,
      top: y,
      right: x + this.bubbleSize,
      bottom: y + this.bubbleSize,
      toJSON: () => ({}),
    } as DOMRect;
  }

  updateRects(bubbles: BubbleData[]): DOMRect[] {
    return bubbles.map((b) => this.generateRect(b.x, b.y));
  }

  checkCollisions(bubblesData: BubbleData[]) {
  const maxIterations = 5;
  const pushBackDistance = -5;

  bubblesData.forEach((b) => (b.isColliding = false));

  for (let iter = 0; iter < maxIterations; iter++) {
    const bubbleRects = this.updateRects(bubblesData);
    let anyCollision = false;

    for (let i = 0; i < bubbleRects.length; i++) {
      for (let j = i + 1; j < bubbleRects.length; j++) {
        const a = bubbleRects[i];
        const b = bubbleRects[j];

        if (this.isColliding(a, b)) {
          anyCollision = true;

          bubblesData[i].isColliding = true;
          bubblesData[j].isColliding = true;

          const dx = (a.left + a.width / 2) - (b.left + b.width / 2);
          const dy = (a.top + a.height / 2) - (b.top + b.height / 2);
          const magnitude = Math.sqrt(dx * dx + dy * dy) || 1;

          const offsetX = (dx / magnitude) * pushBackDistance;
          const offsetY = (dy / magnitude) * pushBackDistance;

          const canMove = (b: BubbleData, offsetX: number, offsetY: number) => {
            const newPos = this.clampPosition(b.x + offsetX, b.y + offsetY);
            return newPos.x !== b.x || newPos.y !== b.y;
          };

          const moveBubble = (b: BubbleData, offsetX: number, offsetY: number) => {
            const newPos = this.clampPosition(b.x + offsetX, b.y + offsetY);
            b.x = newPos.x;
            b.y = newPos.y;
          };

          const aCanMove = canMove(bubblesData[i], -offsetX, -offsetY);
          const bCanMove = canMove(bubblesData[j], offsetX, offsetY);

          if (bubblesData[i].isDragging && !bubblesData[j].isDragging) {
            if (bCanMove) {
              moveBubble(bubblesData[j], offsetX, offsetY);
            } else {
              moveBubble(bubblesData[i], -offsetX, - offsetY);
            }
          } else if (!bubblesData[i].isDragging && bubblesData[j].isDragging) {
            if (aCanMove) {
              moveBubble(bubblesData[i], -offsetX, -offsetY);
            } else {
              moveBubble(bubblesData[j], offsetX, offsetY);
            }
          } else if (!bubblesData[i].isDragging && !bubblesData[j].isDragging) {
            if (aCanMove) moveBubble(bubblesData[i], -offsetX / 2, -offsetY / 2);
            if (bCanMove) moveBubble(bubblesData[j], offsetX / 2, offsetY / 2);
          }
        }
      }
    }

    if (!anyCollision) break;
  }
}

}
