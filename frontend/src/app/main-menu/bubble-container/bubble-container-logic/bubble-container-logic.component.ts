import { HEIGHT, WIDTH, BUBBLESIZE } from "../../main-menuConstants";

///TODO: IMPROVE THE COLLISIONS

export interface BubbleData {
    id: number;
    isColliding: boolean;
    isDragging: boolean;
    x: number;
    y: number;
    prevX?: number;
    prevY?: number;
}

export class BubbleContainerLogic {
    readonly maxIterations = 15;

    constructor() {}

    clampPosition(x: number, y: number) {
        const maxX = WIDTH - BUBBLESIZE;
        const maxY = HEIGHT - BUBBLESIZE;

        return {
            x: Math.min(Math.max(x, 0), maxX),
            y: Math.min(Math.max(y, 0), maxY),
        };
    }

    isColliding(a: DOMRect, b: DOMRect): boolean {
        return !(
            a.right <= b.left ||
            a.left >= b.right ||
            a.bottom <= b.top ||
            a.top >= b.bottom
        );
    }

    generateRect(x: number, y: number): DOMRect {
        return {
            x,
            y,
            width: BUBBLESIZE,
            height: BUBBLESIZE,
            left: x,
            top: y,
            right: x + BUBBLESIZE,
            bottom: y + BUBBLESIZE,
            toJSON: () => ({}),
        } as DOMRect;
    }

    updateRects(bubbles: BubbleData[]): DOMRect[] {
        return bubbles.map((b) => this.generateRect(b.x, b.y));
    }

    checkCollisions(bubblesData: BubbleData[]) {
        for (const b of bubblesData) {
            b.prevX = b.x;
            b.prevY = b.y;
            b.isColliding = false;
        }

        for (let iter = 0; iter < this.maxIterations; iter++) {
            const bubbleRects = this.updateRects(bubblesData);
            let anyCollision = false;

            for (let i = 0; i < bubblesData.length; i++) {
                for (let j = i + 1; j < bubblesData.length; j++) {
                    const rectA = bubbleRects[i];
                    const rectB = bubbleRects[j];

                    if (this.isColliding(rectA, rectB)) {
                        anyCollision = true;
                        const bubbleA = bubblesData[i];
                        const bubbleB = bubblesData[j];

                        bubbleA.isColliding = true;
                        bubbleB.isColliding = true;

                        const centerAX = rectA.left + BUBBLESIZE / 2;
                        const centerAY = rectA.top + BUBBLESIZE / 2;
                        const centerBX = rectB.left + BUBBLESIZE / 2;
                        const centerBY = rectB.top + BUBBLESIZE / 2;

                        const dx = centerBX - centerAX;
                        const dy = centerBY - centerAY;
                        const distance = Math.sqrt(dx * dx + dy * dy) || 1;

                        const minDistance = BUBBLESIZE;
                        const overlap = minDistance - distance;

                        const nx = dx / distance;
                        const ny = dy / distance;

                        const moveA = bubbleA.isDragging ? 0 : 1;
                        const moveB = bubbleB.isDragging ? 0 : 1;
                        const totalMove = moveA + moveB || 1;

                        const offsetAX = -nx * (overlap * (moveA / totalMove));
                        const offsetAY = -ny * (overlap * (moveA / totalMove));
                        const offsetBX = nx * (overlap * (moveB / totalMove));
                        const offsetBY = ny * (overlap * (moveB / totalMove));

                        const canMove = (
                            b: BubbleData,
                            dx: number,
                            dy: number,
                        ) => {
                            const clamped = this.clampPosition(
                                b.x + dx,
                                b.y + dy,
                            );
                            return clamped.x !== b.x || clamped.y !== b.y;
                        };

                        const move = (
                            b: BubbleData,
                            dx: number,
                            dy: number,
                        ) => {
                            const clamped = this.clampPosition(
                                b.x + dx,
                                b.y + dy,
                            );
                            b.x = clamped.x;
                            b.y = clamped.y;
                        };

                        const aCanMove = canMove(bubbleA, offsetAX, offsetAY);
                        const bCanMove = canMove(bubbleB, offsetBX, offsetBY);

                        if (aCanMove) move(bubbleA, offsetAX, offsetAY);
                        if (bCanMove) move(bubbleB, offsetBX, offsetBY);

                        if (!aCanMove && !bCanMove) {
                            const correction = 1;
                            const fallbackAX = -nx * correction;
                            const fallbackAY = -ny * correction;
                            const fallbackBX = nx * correction;
                            const fallbackBY = ny * correction;

                            const fallbackACanMove = canMove(
                                bubbleA,
                                fallbackAX,
                                fallbackAY,
                            );
                            const fallbackBCanMove = canMove(
                                bubbleB,
                                fallbackBX,
                                fallbackBY,
                            );

                            if (fallbackACanMove)
                                move(bubbleA, fallbackAX, fallbackAY);
                            if (fallbackBCanMove)
                                move(bubbleB, fallbackBX, fallbackBY);
                        }
                    }
                }
            }

            if (!anyCollision) break;
        }
    }
}
