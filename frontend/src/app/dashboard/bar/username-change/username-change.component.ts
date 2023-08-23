import {Component, ElementRef, ViewChild} from '@angular/core';
import {ProfileService} from "../../../service/profile.service";

@Component({
  selector: 'app-username-change',
  templateUrl: './username-change.component.html',
  styleUrls: ['./username-change.component.css']
})
export class UsernameChangeComponent {

  @ViewChild('username')
  username!: ElementRef<HTMLInputElement>

  constructor(public profileService: ProfileService) {
  }

  changeUsername() {
    let username = this.username.nativeElement.value

    if (username == '') return

    this.profileService.changeUsername(username)
  }
}
