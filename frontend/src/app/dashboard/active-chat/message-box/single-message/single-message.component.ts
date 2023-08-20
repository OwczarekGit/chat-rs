import {Component, Input} from '@angular/core';
import {ChatMessage} from "../../../../data/chat-message";
import {ProfileService} from "../../../../service/profile.service";

@Component({
  selector: 'app-single-message',
  templateUrl: './single-message.component.html',
  styleUrls: ['./single-message.component.css']
})
export class SingleMessageComponent {

  @Input()
  public message!: ChatMessage

  constructor(public profileService: ProfileService) {}

}
