import { Component, ViewChild } from '@angular/core';
import { CdkDrag } from '@angular/cdk/drag-drop';
import { BubbleComponent } from "../bubble/bubble.component";

@Component({
  selector: 'bubbleContainer',
  templateUrl: "bubbleContainer.component.html",
  styleUrl: "bubbleContainer.component.scss",
  standalone: true,
  imports: [BubbleComponent],
})
export class BubbleContainer {
    boundarySelector = '.boundary';
}