import { Component } from '@angular/core';
import { BubbleComponent } from "./bubble/bubble.component";
import { BubbleContainer } from "./bubbleContainer/bubbleContainer.component";

@Component({
  selector: 'mainMenu',
  standalone: true,
  imports: [BubbleComponent, BubbleContainer],
  templateUrl: './mainMenu.component.html'
})
export class MainMenuComponent {}