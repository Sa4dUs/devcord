import {
  Component,
  ViewChildren,
  QueryList,
  AfterViewInit,
  ElementRef,
  ViewChild,
} from '@angular/core';
import { BubbleComponent } from '../bubble/bubble.component';
import { CommonModule } from '@angular/common';
import { ImageCropperComponent, ImageCroppedEvent } from 'ngx-image-cropper';

@Component({
  selector: 'bubbleContainer',
  templateUrl: 'bubbleContainer.component.html',
  styleUrls: ['bubbleContainer.component.scss'],
  standalone: true,
  imports: [CommonModule, BubbleComponent, ImageCropperComponent],
})
export class BubbleContainer implements AfterViewInit {
  boundarySelector = '.example-boundary';

  bubbleRects: DOMRect[] = [];

  bubblesData = [
    { id: 1, isColliding: false, isDragging: false, x: 0, y: 0 },
  ];

  @ViewChildren(BubbleComponent) bubbles!: QueryList<BubbleComponent>;
  @ViewChild('fileInput') fileInput!: ElementRef<HTMLInputElement>;

  boundaryRect!: DOMRect;

  imageChangedEvent: any = '';
  croppedImage: string = '';
  showCropper = false;

  backgroundStyle = '';

  constructor(private host: ElementRef) {}

  openFileSelector() {
    if (this.fileInput) {
      this.fileInput.nativeElement.value = '';
      this.fileInput.nativeElement.click();
    }
  }

fileChangeEvent(event: any): void {
  this.imageChangedEvent = event;
  this.showCropper = true;
}

imageCropped(event: ImageCroppedEvent) {
  if (event.base64) {
    this.croppedImage = event.base64;
  } else if (event.blob) {
    const reader = new FileReader();
    reader.onload = () => {
      this.croppedImage = reader.result as string;
    };
    reader.readAsDataURL(event.blob);
  } else {
    this.croppedImage = '';
  }
}

applyCrop() {
  if (this.croppedImage) {
    this.backgroundStyle = `url('${this.croppedImage}')`;
    this.showCropper = false;
    this.imageChangedEvent = '';
  }
}


cancelCrop() {
  this.showCropper = false;
  this.imageChangedEvent = '';
  this.croppedImage = '';
}

  ngAfterViewInit() {
    const boundaryEl = this.host.nativeElement.querySelector(this.boundarySelector);
    if (boundaryEl) {
      this.boundaryRect = boundaryEl.getBoundingClientRect();
    }
  }

  onBubbleMoved(index: number, pos: { x: number; y: number }) {
    const clampedPos = this.clampPosition(pos.x, pos.y);
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
    this.bubbleRects[index] = {
      x: b.x,
      y: b.y,
      width: 100,
      height: 100,
      left: b.x,
      top: b.y,
      right: b.x + 100,
      bottom: b.y + 100,
      toJSON: () => ({}),
    } as DOMRect;
  }

  clampPosition(x: number, y: number) {
    const minX = 0;
    const minY = 0;
    const maxX = (this.boundaryRect?.width || 400) - 100;
    const maxY = (this.boundaryRect?.height || 400) - 100;

    return {
      x: Math.min(Math.max(x, minX), maxX),
      y: Math.min(Math.max(y, minY), maxY),
    };
  }

  checkCollisions() {
    const maxIterations = 5;
    const pushBackDistance = -5;

    this.bubblesData.forEach((b) => (b.isColliding = false));

    for (let iter = 0; iter < maxIterations; iter++) {
      this.bubbleRects = this.bubblesData.map((b) => ({
        x: b.x,
        y: b.y,
        width: 100,
        height: 100,
        left: b.x,
        top: b.y,
        right: b.x + 100,
        bottom: b.y + 100,
        toJSON: () => ({}),
      }) as DOMRect);

      let anyCollision = false;

      for (let i = 0; i < this.bubbleRects.length; i++) {
        for (let j = i + 1; j < this.bubbleRects.length; j++) {
          const a = this.bubbleRects[i];
          const b = this.bubbleRects[j];

          if (this.isColliding(a, b)) {
            anyCollision = true;

            this.bubblesData[i].isColliding = true;
            this.bubblesData[j].isColliding = true;

            const dx = (a.left + a.width / 2) - (b.left + b.width / 2);
            const dy = (a.top + a.height / 2) - (b.top + b.height / 2);
            const magnitude = Math.sqrt(dx * dx + dy * dy) || 1;

            const offsetX = (dx / magnitude) * pushBackDistance;
            const offsetY = (dy / magnitude) * pushBackDistance;

            if (this.bubblesData[i].isDragging && !this.bubblesData[j].isDragging) {
              const newPos = this.clampPosition(
                this.bubblesData[j].x + offsetX,
                this.bubblesData[j].y + offsetY
              );
              this.bubblesData[j].x = newPos.x;
              this.bubblesData[j].y = newPos.y;
            } else if (!this.bubblesData[i].isDragging && this.bubblesData[j].isDragging) {
              const newPos = this.clampPosition(
                this.bubblesData[i].x - offsetX,
                this.bubblesData[i].y - offsetY
              );
              this.bubblesData[i].x = newPos.x;
              this.bubblesData[i].y = newPos.y;
            } else if (!this.bubblesData[i].isDragging && !this.bubblesData[j].isDragging) {
              const newPosI = this.clampPosition(
                this.bubblesData[i].x - offsetX / 2,
                this.bubblesData[i].y - offsetY / 2
              );
              const newPosJ = this.clampPosition(
                this.bubblesData[j].x + offsetX / 2,
                this.bubblesData[j].y + offsetY / 2
              );
              this.bubblesData[i].x = newPosI.x;
              this.bubblesData[i].y = newPosI.y;
              this.bubblesData[j].x = newPosJ.x;
              this.bubblesData[j].y = newPosJ.y;
            }
          }
        }
      }

      if (!anyCollision) break;
    }
  }

  isColliding(a: DOMRect, b: DOMRect): boolean {
    return !(
      a.right < b.left ||
      a.left > b.right ||
      a.bottom < b.top ||
      a.top > b.bottom
    );
  }

  trackById(index: number, item: { id: number }) {
    return item.id;
  }

  addBubble() {
    const newId = this.bubblesData.length + 1;
    const maxAttempts = 100;
    for (let attempt = 0; attempt < maxAttempts; attempt++) {
      const x = Math.random() * ((this.boundaryRect?.width || 400) - 100);
      const y = Math.random() * ((this.boundaryRect?.height || 400) - 100);
      const fakeRect = {
        x,
        y,
        width: 100,
        height: 100,
        left: x,
        top: y,
        right: x + 100,
        bottom: y + 100,
        toJSON: () => ({}),
      } as DOMRect;

      const collision = this.bubbleRects.some((r) => this.isColliding(r, fakeRect));
      if (!collision) {
        this.bubbleRects.push(fakeRect);
        this.bubblesData.push({ id: newId, isColliding: false, isDragging: false, x, y });
        break;
      }
    }
  }
}
