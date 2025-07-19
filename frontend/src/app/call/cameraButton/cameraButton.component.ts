import { Component, Output, EventEmitter } from '@angular/core';
import { CommonModule } from '@angular/common';

@Component({
  selector: 'cameraButton',
  standalone: true,
  imports: [CommonModule],
  templateUrl: './cameraButton.component.html',
  styleUrls: ['./cameraButton.component.scss']
})
export class CameraButton {
  @Output() capture = new EventEmitter<void>();

  onCapture(): void {
    this.capture.emit();
  }
}
