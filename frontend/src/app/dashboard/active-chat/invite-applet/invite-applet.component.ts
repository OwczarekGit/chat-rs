import {Component, ElementRef, ViewChild} from '@angular/core';
import {SearchService} from "../../../service/search.service";
import {ProfileSearchResult} from "../../../data/profile-search-result";

@Component({
  selector: 'app-invite-applet',
  templateUrl: './invite-applet.component.html',
  styleUrls: ['./invite-applet.component.css']
})
export class InviteAppletComponent {

  @ViewChild("applet")
  applet!: ElementRef<HTMLDivElement>

  @ViewChild("search_field")
  searchField!: ElementRef<HTMLInputElement>

  searchResults: ProfileSearchResult[] = []

  constructor(private searchService: SearchService) {
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
}
