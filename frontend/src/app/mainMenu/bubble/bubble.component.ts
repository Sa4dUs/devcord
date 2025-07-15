import {
  Component,
  Input,
  Output,
  EventEmitter,
  ElementRef,
  AfterViewInit,
  OnChanges,
  SimpleChanges,
} from '@angular/core';
import {
  CdkDragEnd,
  CdkDragStart,
  CdkDrag,
  CdkDragMove,
} from '@angular/cdk/drag-drop';

@Component({
  selector: 'bubble',
  templateUrl: './bubble.component.html',
  styleUrls: ['./bubble.component.scss'],
  standalone: true,
  imports: [CdkDrag],
})
export class BubbleComponent implements AfterViewInit, OnChanges {
  @Input() id!: number;
  @Input() boundarySelector = '';
  @Input() isColliding = false;
  @Input() isDragging = false;
  @Input() x = 0;
  @Input() y = 0;

  @Output() positionChanged = new EventEmitter<{ x: number; y: number }>();
  @Output() dragStateChanged = new EventEmitter<boolean>();

  constructor(private el: ElementRef) {}

  ngAfterViewInit() {
    this.setPosition();
    this.emitPosition();
  }

  ngOnChanges(changes: SimpleChanges) {
    if (changes['x'] || changes['y']) {
      this.setPosition();
    }
  }

  onDragStarted(event: CdkDragStart) {
    this.isDragging = true;
    this.dragStateChanged.emit(true);
  }

  onDragEnded(event: CdkDragEnd) {
    this.isDragging = false;
    this.dragStateChanged.emit(false);

    const bubbleEl = this.el.nativeElement.querySelector('.bubble');
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
    const bubbleEl = this.el.nativeElement.querySelector('.bubble');
    if (bubbleEl) {
      bubbleEl.style.left = `${this.x}px`;
      bubbleEl.style.top = `${this.y}px`;
      bubbleEl.style.transform = '';
    }
  }
}
