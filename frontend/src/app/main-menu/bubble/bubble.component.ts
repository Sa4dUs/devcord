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
    @Input() x = 0;
    @Input() y = 0;

    @Output() positionChanged = new EventEmitter<{ x: number; y: number }>();
    @Output() dragStateChanged = new EventEmitter<boolean>();

    bubblesize = BUBBLESIZE;

    private fakeClick = false;
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

    onClick(event: MouseEvent) {
        if (this.fakeClick) {
            this.fakeClick = false;
            return;
        }
        if (this.dragOccurred) {
            event.stopImmediatePropagation();
            return;
        }
        this.router.navigate(["/group", this.id]);
    }
    // eslint-disable-next-line
    onDragStarted(event: CdkDragStart) {
        this.dragOccurred = false;
        this.addDragAnimation();
        this.dragStateChanged.emit(true);
    }

    onDragMoved(event: CdkDragMove) {
        this.dragOccurred = true;
        const pos = event.pointerPosition;
        const boundaryEl = document.querySelector(this.boundarySelector);

        if (boundaryEl) {
            const rect = boundaryEl.getBoundingClientRect();
            let newX = pos.x - rect.left - this.bubblesize / 2;
            let newY = pos.y - rect.top - this.bubblesize / 2;

            newX = Math.min(Math.max(newX, 0), rect.width - this.bubblesize);
            newY = Math.min(Math.max(newY, 0), rect.height - this.bubblesize);

            this.x = newX;
            this.y = newY;

            this.setPosition();
            this.emitPosition();
        }
    }
    // eslint-disable-next-line
    onDragEnded(event: CdkDragEnd) {
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
        this.dragOccurred = false;
        this.fakeClick = true;
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
        el?.classList.add("dragging");
    }

    private removeDragAnimation() {
        const el = this.el.nativeElement.querySelector(".bubble");
        el?.classList.remove("dragging");
    }

    private setBubbleSizeVariable() {
        const el = this.el.nativeElement.querySelector(".bubble");
        el?.style.setProperty("--bubblesize", this.bubblesize.toString());
    }
}
