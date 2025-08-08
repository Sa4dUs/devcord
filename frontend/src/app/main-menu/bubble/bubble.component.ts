import {
    Component,
    Input,
    Output,
    EventEmitter,
    ElementRef,
    AfterViewInit,
    OnChanges,
    SimpleChanges,
    inject,
} from "@angular/core";
import {
    CdkDragEnd,
    CdkDragStart,
    CdkDrag,
    CdkDragMove,
} from "@angular/cdk/drag-drop";
import { BUBBLESIZE } from "../main-menuConstants";
import { Router } from "@angular/router";

@Component({
    selector: "bubble",
    templateUrl: "./bubble.component.html",
    styleUrls: ["./bubble.component.scss"],
    standalone: true,
    imports: [CdkDrag],
})
export class BubbleComponent implements AfterViewInit, OnChanges {
    @Input() id!: string;
    @Input() boundarySelector = "";
    @Input() isColliding = false;
    @Input() isDragging = false;
    @Input() x = 0;
    @Input() y = 0;

    @Output() positionChanged = new EventEmitter<{ x: number; y: number }>();
    @Output() dragStateChanged = new EventEmitter<boolean>();
    @Output() clicked = new EventEmitter<void>();

    bubblesize = BUBBLESIZE;

    private mouseDownTime: number = 0;
    private clickThreshold = 150;
    private dragOccurred = false;

    private router = inject(Router);

    constructor(private el: ElementRef) {}

    ngAfterViewInit() {
        this.setPosition();
        this.emitPosition();
        this.setBubbleSizeVariable();
    }

    ngOnChanges(changes: SimpleChanges) {
        if (changes["x"] || changes["y"]) {
            this.setPosition();
        }
    }

    onMouseDown() {
        this.mouseDownTime = Date.now();
        this.dragOccurred = false;
    }

    onMouseUp() {
        const heldTime = Date.now() - this.mouseDownTime;
        if (heldTime > this.clickThreshold && this.isDragging) {
            this.dragOccurred = true;
        }
    }

    onClick() {
        if (!this.dragOccurred && !this.isDragging) {
            this.router.navigate(["/group", this.id]);
        }
    }
    // eslint-disable-next-line @typescript-eslint/no-unused-vars
    onDragStarted(event: CdkDragStart) {
        this.isDragging = true;
        this.addDragAnimation();
        this.dragStateChanged.emit(true);
    }
    // eslint-disable-next-line @typescript-eslint/no-unused-vars
    onDragEnded(event: CdkDragEnd) {
        this.isDragging = false;
        this.dragOccurred = false;
        this.removeDragAnimation();
        this.dragStateChanged.emit(false);

        const bubbleEl = this.el.nativeElement.querySelector(".bubble");
        if (bubbleEl) {
            const left = parseFloat(bubbleEl.style.left) || 0;
            const top = parseFloat(bubbleEl.style.top) || 0;
            this.x = left;
            this.y = top;
        }

        this.emitPosition();
    }

    onDragMoved(event: CdkDragMove) {
        const pos = event.pointerPosition;
        const boundaryEl = document.querySelector(this.boundarySelector);

        if (boundaryEl) {
            const rect = boundaryEl.getBoundingClientRect();

            let newX = pos.x - rect.left - 50;
            let newY = pos.y - rect.top - 50;

            newX = Math.min(Math.max(newX, 0), rect.width - 100);
            newY = Math.min(Math.max(newY, 0), rect.height - 100);

            this.x = newX;
            this.y = newY;

            this.setPosition();
            this.emitPosition();
        }
    }

    private emitPosition() {
        this.positionChanged.emit({ x: this.x, y: this.y });
    }

    private setPosition() {
        const bubbleEl = this.el.nativeElement.querySelector(".bubble");
        if (bubbleEl) {
            bubbleEl.style.left = `${this.x}px`;
            bubbleEl.style.top = `${this.y}px`;
            bubbleEl.style.transform = "";
        }
    }

    private addDragAnimation() {
        const el = this.el.nativeElement.querySelector(".bubble");
        if (el) {
            el.classList.add("dragging");
        }
    }

    private removeDragAnimation() {
        const el = this.el.nativeElement.querySelector(".bubble");
        if (el) {
            el.classList.remove("dragging");
        }
    }

    private setBubbleSizeVariable() {
        const el = this.el.nativeElement.querySelector(".bubble");
        if (el) {
            el.style.setProperty("--bubblesize", this.bubblesize.toString());
        }
    }
}
