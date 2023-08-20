import {Component, ElementRef, Input, ViewChild} from '@angular/core';
import {SearchService} from "../../../service/search.service";
import {ProfileSearchResult} from "../../../data/profile-search-result";
import {ChatService} from "../../../service/chat.service";

@Component({
  selector: 'app-invite-applet',
  templateUrl: './invite-applet.component.html',
  styleUrls: ['./invite-applet.component.css']
})
export class InviteAppletComponent {

  @Input()
  activeChatId!: number

  @ViewChild("applet")
  applet!: ElementRef<HTMLDivElement>

  @ViewChild("search_field")
  searchField!: ElementRef<HTMLInputElement>

  searchResults: ProfileSearchResult[] = []

  constructor(private searchService: SearchService, private chatService: ChatService) {
  }

  toggle() {
    this.applet.nativeElement.classList.toggle('hidden')
  }

  performSearch() {
    let phrase = this.searchField.nativeElement.value
    if (phrase == '') return

    this.searchService.searchUsers(phrase).subscribe({
      next: value => {
        this.searchResults = value;
      }
    })
  }

  inviteMember(r: ProfileSearchResult) {
    this.applet.nativeElement.classList.add('hidden')
    this.chatService.inviteToChat(this.activeChatId, r.id)
  }
}
