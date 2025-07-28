import {
    Component,
    ElementRef,
    ViewChild,
    Input,
    Output,
    EventEmitter,
} from "@angular/core";
import { CommonModule } from "@angular/common";
import { ImageCropperComponent, ImageCroppedEvent } from "ngx-image-cropper";

@Component({
    selector: "bubbleContainerBackground",
    standalone: true,
    imports: [CommonModule, ImageCropperComponent],
    templateUrl: "./bubbleContainerBackground.component.html",
    styleUrls: ["./bubbleContainerBackground.component.scss"],
})
export class BubbleContainerBackground {
    @Input() boundarySelector = ".example-boundary";
    @ViewChild("fileInput") fileInput!: ElementRef<HTMLInputElement>;

    @Input() boundaryWidth: number = 400;
    @Input() boundaryHeight: number = 400;

    @Output() backgroundSelected = new EventEmitter<string>();

    imageChangedEvent: Event | null = null;
    croppedImage: string = "";
    showCropper = false;

    originalImageWidth = 0;
    originalImageHeight = 0;

    backgroundStyle = "";

    openFileSelector() {
        if (this.fileInput) {
            this.fileInput.nativeElement.value = "";
            this.fileInput.nativeElement.click();
        }
    }

    fileChangeEvent(event: Event): void {
        this.imageChangedEvent = event;
        this.showCropper = true;

        const input = event.target as HTMLInputElement;
        const file = input.files?.[0];
        if (!file) return;

        const img = new Image();
        const reader = new FileReader();

        reader.onload = (e) => {
            img.onload = () => {
                this.originalImageWidth = img.width;
                this.originalImageHeight = img.height;
            };
            img.src = e.target?.result as string;
        };

        reader.readAsDataURL(file);
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
            this.croppedImage = "";
        }
    }

    applyCrop() {
        if (this.croppedImage) {
            this.backgroundSelected.emit(this.croppedImage);
            this.showCropper = false;
            this.imageChangedEvent = null;
        }
    }

    cancelCrop() {
        this.showCropper = false;
        this.imageChangedEvent = null;
        this.croppedImage = "";
    }

    removeBackground() {
        this.backgroundSelected.emit("");
    }

    changeBackgroundColor(event: Event) {
        const input = event.target as HTMLInputElement;
        const color = input.value;
        this.backgroundSelected.emit(color);
    }
}
