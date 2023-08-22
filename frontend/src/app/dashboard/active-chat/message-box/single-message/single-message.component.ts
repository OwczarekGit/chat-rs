import {Component, Inject, Input, LOCALE_ID} from '@angular/core';
import {ChatMessage} from "../../../../data/chat-message";
import {ProfileService} from "../../../../service/profile.service";
import {formatDate} from "@angular/common";

@Component({
  selector: 'app-single-message',
  templateUrl: './single-message.component.html',
  styleUrls: ['./single-message.component.css']
})
export class SingleMessageComponent {

  @Input()
  public message!: ChatMessage

  constructor(public profileService: ProfileService, @Inject(LOCALE_ID) public locale: string) {}

  public formatDate(): string {
    let c = this.message.created
    let date = formatDate(c, 'dd.MM.yyyy, hh:mm UTC', this.locale);
    return date.toString()
  }

}
