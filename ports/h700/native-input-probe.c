/* Small native SDL2 input diagnostic for SP PortMaster testing. */
#include <SDL2/SDL.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
static const unsigned char font[36][7]={
{14,17,17,31,17,17,17},{30,17,17,30,17,17,30},{14,17,16,16,16,17,14},{30,17,17,17,17,17,30},
{31,16,16,30,16,16,31},{31,16,16,30,16,16,16},{14,17,16,23,17,17,15},{17,17,17,31,17,17,17},
{14,4,4,4,4,4,14},{7,2,2,2,2,18,12},{17,18,20,24,20,18,17},{16,16,16,16,16,16,31},
{17,27,21,21,17,17,17},{17,25,21,19,17,17,17},{14,17,17,17,17,17,14},{30,17,17,30,16,16,16},
{14,17,17,17,21,18,13},{30,17,17,30,20,18,17},{15,16,16,14,1,1,30},{31,4,4,4,4,4,4},
{17,17,17,17,17,17,14},{17,17,17,17,17,10,4},{17,17,17,21,21,21,10},{17,17,10,4,10,17,17},
{17,17,10,4,4,4,4},{31,1,2,4,8,16,31},
{14,17,19,21,25,17,14},{4,12,4,4,4,4,14},{14,17,1,2,4,8,31},{30,1,1,14,1,1,30},
{2,6,10,18,31,2,2},{31,16,16,30,1,1,30},{14,16,16,30,17,17,14},{31,1,2,4,8,8,8},
{14,17,17,14,17,17,14},{14,17,17,15,1,1,14}};
static void text(SDL_Renderer*r,int x,int y,int scale,const char*s){
 SDL_SetRenderDrawColor(r,205,205,205,255);
 for(;*s;s++,x+=6*scale){const unsigned char*g=NULL;
  if(*s>='A'&&*s<='Z')g=font[*s-'A'];else if(*s>='0'&&*s<='9')g=font[26+*s-'0'];
  for(int row=0;row<7;row++)for(int col=0;col<5;col++){
   int on=g?(g[row]&(1<<(4-col))):(*s=='-'&&row==3)||(*s==':'&&col==2&&(row==2||row==5))||(*s=='+'&&(row==3||(col==2&&row>0&&row<6)));
   if(on){SDL_Rect q={x+col*scale,y+row*scale,scale,scale};SDL_RenderFillRect(r,&q);}
  }
 }
}
int main(int argc,char**argv){
 int test=argc>1&&!strcmp(argv[1],"--self-test");
 SDL_SetHint(SDL_HINT_JOYSTICK_ALLOW_BACKGROUND_EVENTS,"1");
 if(SDL_Init(SDL_INIT_VIDEO|SDL_INIT_JOYSTICK)<0){fprintf(stderr,"controls test: %s\n",SDL_GetError());return 1;}
 SDL_Window*w=SDL_CreateWindow("Slot Controls Test",0,0,720,480,test?0:SDL_WINDOW_FULLSCREEN);
 if(!w){fprintf(stderr,"controls test window: %s\n",SDL_GetError());return 1;}
 SDL_Renderer*r=SDL_CreateRenderer(w,-1,test?SDL_RENDERER_SOFTWARE:SDL_RENDERER_ACCELERATED);
 if(!r)r=SDL_CreateRenderer(w,-1,SDL_RENDERER_SOFTWARE);
 if(!r){fprintf(stderr,"controls test renderer: %s\n",SDL_GetError());return 1;}
 SDL_RenderSetLogicalSize(r,720,480);
 fprintf(stderr,"controls test: joysticks=%d\n",SDL_NumJoysticks());
 SDL_Joystick*pad=SDL_NumJoysticks()?SDL_JoystickOpen(0):NULL;
 if(pad)fprintf(stderr,"controls test: %s axes=%d hats=%d buttons=%d\n",SDL_JoystickName(pad),SDL_JoystickNumAxes(pad),SDL_JoystickNumHats(pad),SDL_JoystickNumButtons(pad));
 int running=1,lastx=99999,lasty=99999,lasthat=-1;unsigned frames=0;
 while(running){SDL_Event ev;while(SDL_PollEvent(&ev))if(ev.type==SDL_QUIT)running=0;
  SDL_JoystickUpdate();int x=pad?SDL_JoystickGetAxis(pad,0):0,y=pad?SDL_JoystickGetAxis(pad,1):0;
  int hat=pad?SDL_JoystickGetHat(pad,0):0;
  if(pad&&(x!=lastx||y!=lasty||hat!=lasthat)){fprintf(stderr,"controls test: hat=%d left-x=%d left-y=%d\n",hat,x,y);lastx=x;lasty=y;lasthat=hat;}
  SDL_SetRenderDrawColor(r,12,12,12,255);SDL_RenderClear(r);
  text(r,24,24,3,"SLOT CONTROLS TEST");
  text(r,24,66,2,"MENU + SELECT: SWITCH DPAD AND LEFT STICK");
  text(r,24,94,2,"HOLD MENU ALONE TO RETURN TO SLOT");
  char line[80];snprintf(line,sizeof(line),"DPAD HAT: %d",hat);text(r,24,145,3,line);
  snprintf(line,sizeof(line),"LEFT STICK X: %d",x);text(r,24,185,3,line);
  snprintf(line,sizeof(line),"LEFT STICK Y: %d",y);text(r,24,225,3,line);
  text(r,24,277,2,pad?"HOLD A DIRECTION AND TRY BOTH MODES":"NO CONTROLLER FOUND - RECONNECT CARD");
  text(r,24,310,2,"DIGITAL: HAT CHANGES - STICK STAYS ZERO");
  text(r,24,338,2,"ANALOG: STICK CHANGES - HAT STAYS ZERO");
  for(int i=0;i<11;i++){
   int on=pad&&SDL_JoystickGetButton(pad,i);SDL_Rect q={24+i*60,385,46,46};
   SDL_SetRenderDrawColor(r,on?125:35,on?125:35,on?125:35,255);SDL_RenderFillRect(r,&q);
   snprintf(line,sizeof(line),"%d",i);text(r,q.x+10,q.y+15,2,line);
  }
  SDL_RenderPresent(r);SDL_Delay(16);frames++;
  if(test&&frames==5)break;
 }
 if(pad)SDL_JoystickClose(pad);
 SDL_DestroyRenderer(r);SDL_DestroyWindow(w);SDL_Quit();return 0;
}
