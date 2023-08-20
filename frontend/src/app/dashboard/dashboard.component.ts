import {AfterViewInit, Component, ElementRef, ViewChild} from '@angular/core';
import {ChatMessage} from "../data/chat-message";
import {NotificationService} from "../service/notification.service";
import {UtilService} from "../service/util.service";
import {ActivatedRoute} from "@angular/router";

@Component({
  selector: 'app-dashboard',
  templateUrl: './dashboard.component.html',
  styleUrls: ['./dashboard.component.css']
})
export class DashboardComponent implements AfterViewInit {

  @ViewChild("content")
  content!: ElementRef<HTMLDivElement>

  public mobileMode: boolean = false
  public contentFocused: boolean = false

  constructor(
    private notificationSubject: NotificationService, private utilService: UtilService) {
    this.utilService.backButtonClicked.subscribe({
      next: v => {
        this.contentFocused = false
        this.updateView()
      }
    })
  }

  ngAfterViewInit() {
    let observer = new ResizeObserver((obs) => {
      let w = obs[0].contentRect.width

      this.mobileMode = w < 700
      this.updateView()
    })

    observer.observe(this.content.nativeElement)
  }

  public updateView() {
    if (this.mobileMode) {
      if (this.contentFocused) {
        this.onlyShowContent()
      } else {
        this.onlyShowPanel()
      }
    } else {
      this.showSideBySide()
    }
  }

  public showSideBySide() {
    this.content.nativeElement.style.gridTemplateColumns = '240px 1fr'
  }

  public onlyShowPanel() {
    this.content.nativeElement.style.gridTemplateColumns = '1fr 0'
  }

  public onlyShowContent() {
    this.content.nativeElement.style.gridTemplateColumns = '0 1fr'
  }

  focusChat() {
    this.contentFocused = true
    this.updateView()
  }
}
