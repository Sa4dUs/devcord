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
import { GroupLoader } from "./group-loader/group-loader.component";

@Component({
    selector: "bubble-container",
    templateUrl: "bubble-container.component.html",
    styleUrls: ["bubble-container.component.scss"],
    standalone: true,
    imports: [
        CommonModule,
        BubbleComponent,
        BubbleContainerBackground,
        GroupLoader,
    ],
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

    bubblesData: BubbleData[] = [];

    logic = new BubbleContainerLogic();

    constructor(private host: ElementRef) {}

    ngAfterViewInit() {
        this.setCssVariables();

        //This avoid a strange error about how angular renders
        if (typeof window !== "undefined" && "DOMRect" in window) {
            this.boundaryRect = new DOMRect(0, 0, this.width, this.height);
        } else {
            this.boundaryRect = {
                x: 0,
                y: 0,
                width: this.width,
                height: this.height,
            } as DOMRect;
        }
    }

    setCssVariables() {
        const el = this.host.nativeElement.querySelector(
            this.boundarySelector,
        ) as HTMLElement | null;

        if (el instanceof HTMLElement) {
            el.style.setProperty("--width", `${this.width}px`);
            el.style.setProperty("--height", `${this.height}px`);
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
        this.checkCollisions();
    }

    onBubbleDragStateChanged(index: number, isDragging: boolean) {
        this.bubblesData[index].isDragging = isDragging;
        this.checkCollisions();
    }

    checkCollisions() {
        this.logic.checkCollisions(this.bubblesData);
    }

    onGroupsLoaded(groups: { groupId: string }[]) {
        groups.forEach((group) => {
            this.addBubble(group.groupId);
        });
    }

    addBubble(groupId: string) {
        const maxAttempts = 100;

        for (let attempt = 0; attempt < maxAttempts; attempt++) {
            const x =
                Math.random() *
                ((this.boundaryRect?.width || this.width) - BUBBLESIZE);
            const y =
                Math.random() *
                ((this.boundaryRect?.height || this.height) - BUBBLESIZE);

            const newBubble: BubbleData = {
                id: groupId,
                isColliding: false,
                isDragging: false,
                x,
                y,
            };

            const collision = this.bubblesData.some((existing) =>
                this.logic.isColliding(existing, newBubble),
            );

            if (!collision) {
                this.bubblesData.push(newBubble);
                break;
            }
        }
    }

    trackById(index: number, item: { id: string }) {
        return item.id;
    }
}
