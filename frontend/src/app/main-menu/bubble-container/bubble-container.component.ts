import {
    Component,
    ViewChildren,
    QueryList,
    AfterViewInit,
    ElementRef,
    ViewChild,
} from "@angular/core";
import { CommonModule } from "@angular/common";
import { BubbleComponent } from "../bubble/bubble.component";
import {
    BubbleContainerLogic,
    BubbleData,
} from "./bubble-container-logic/bubble-container-logic.component";
import { BubbleContainerBackground } from "./bubble-container-background/bubble-container-background.component";
import { WIDTH, HEIGHT, BUBBLESIZE } from "../main-menuConstants";

@Component({
    selector: "bubbleContainer",
    templateUrl: "bubble-container.component.html",
    styleUrls: ["bubble-container.component.scss"],
    standalone: true,
    imports: [CommonModule, BubbleComponent, BubbleContainerBackground],
})
export class BubbleContainer implements AfterViewInit {
    boundarySelector = ".example-boundary";

    @ViewChildren(BubbleComponent) bubbles!: QueryList<BubbleComponent>;
    @ViewChild("bgManager") bgManager!: BubbleContainerBackground;

    boundaryRect!: DOMRect;

    width = WIDTH;
    height = HEIGHT;

    backgroundStyle: string = "";

    transform = {
        scale: 1,
        rotate: 0,
        flipH: false,
        flipV: false,
    };

    bubblesData: BubbleData[] = [
        { id: 1, isColliding: false, isDragging: false, x: 0, y: 0 },
    ];

    bubbleRects: DOMRect[] = [];
    logic = new BubbleContainerLogic();

    constructor(private host: ElementRef) {}

    ngAfterViewInit() {
        this.setCssVariables();
    }

    setCssVariables() {
        const el = this.host.nativeElement.querySelector(this.boundarySelector);
        if (el) {
            el.style.setProperty("--width", this.width + "px");
            el.style.setProperty("--height", this.height + "px");
        }
    }

    onBackgroundSelected(value: string) {
        if (value.startsWith("data:image")) {
            this.backgroundStyle = `url('${value}')`;
        } else {
            this.backgroundStyle = value;
        }
    }

    onBubbleMoved(index: number, pos: { x: number; y: number }) {
        const clampedPos = this.logic.clampPosition(pos.x, pos.y);
        this.bubblesData[index].x = clampedPos.x;
        this.bubblesData[index].y = clampedPos.y;

        this.updateBubbleRect(index);
        this.checkCollisions();
    }

    onBubbleDragStateChanged(index: number, isDragging: boolean) {
        this.bubblesData[index].isDragging = isDragging;
        this.checkCollisions();
    }

    updateBubbleRect(index: number) {
        const b = this.bubblesData[index];
        this.bubbleRects[index] = this.logic.generateRect(b.x, b.y);
    }

    checkCollisions() {
        this.logic.checkCollisions(this.bubblesData);
        this.bubbleRects = this.logic.updateRects(this.bubblesData);
    }

    addBubble() {
        const newId = this.bubblesData.length + 1;
        const maxAttempts = 100;

        for (let attempt = 0; attempt < maxAttempts; attempt++) {
            const x =
                Math.random() *
                ((this.boundaryRect?.width || this.width) - BUBBLESIZE);
            const y =
                Math.random() *
                ((this.boundaryRect?.height || this.height) - BUBBLESIZE);

            const fakeRect = this.logic.generateRect(x, y);
            const collision = this.bubbleRects.some((r) =>
                this.logic.isColliding(r, fakeRect),
            );

            if (!collision) {
                this.bubblesData.push({
                    id: newId,
                    isColliding: false,
                    isDragging: false,
                    x,
                    y,
                });
                this.bubbleRects.push(fakeRect);
                break;
            }
        }
    }

    trackById(index: number, item: { id: number }) {
        return item.id;
    }
}
