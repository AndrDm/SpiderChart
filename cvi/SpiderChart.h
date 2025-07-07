/**************************************************************************/
/* LabWindows/CVI User Interface Resource (UIR) Include File              */
/*                                                                        */
/* WARNING: Do not add to, delete from, or otherwise modify the contents  */
/*          of this include file.                                         */
/**************************************************************************/

#include <userint.h>

#ifdef __cplusplus
    extern "C" {
#endif

     /* Panels and Controls: */

#define  PANEL                            1       /* callback function: main_panel_callback */
#define  PANEL_DETECTOR_STRING            2       /* control type: string, callback function: on_change_callback */
#define  PANEL_IQI_TYPE                   3       /* control type: ring, callback function: on_change_callback */
#define  PANEL_ISRB                       4       /* control type: numeric, callback function: on_change_callback */
#define  PANEL_CSA                        5       /* control type: numeric, callback function: on_change_callback */
#define  PANEL_LAG                        6       /* control type: numeric, callback function: on_change_callback */
#define  PANEL_SNRN                       7       /* control type: numeric, callback function: on_change_callback */
#define  PANEL_SMTR                       8       /* control type: numeric, callback function: on_change_callback */
#define  PANEL_ISOMTL                     9       /* control type: numeric, callback function: on_change_callback */
#define  PANEL_THEME                      10      /* control type: ring, callback function: on_change_callback */
#define  PANEL_QUITBUTTON                 11      /* control type: command, callback function: quit_callback */
#define  PANEL_ISRB_CLASS                 12      /* control type: string, callback function: (none) */
#define  PANEL_CSA_CLASS                  13      /* control type: string, callback function: (none) */
#define  PANEL_LAG_CLASS                  14      /* control type: string, callback function: (none) */
#define  PANEL_SNRN_CLASS                 15      /* control type: string, callback function: (none) */
#define  PANEL_SMTR_CLASS                 16      /* control type: string, callback function: (none) */
#define  PANEL_CANVAS                     17      /* control type: canvas, callback function: scratch_pad */
#define  PANEL_SAVE_AS_BUTTON             18      /* control type: command, callback function: save_as_callback */
#define  PANEL_EXPORT_BUTTON              19      /* control type: command, callback function: export_callback */
#define  PANEL_LOAD_BUTTON                20      /* control type: command, callback function: load_callback */
#define  PANEL_ISOMTL_CLASS               21      /* control type: string, callback function: (none) */
#define  PANEL_LISTBOX                    22      /* control type: listBox, callback function: listbox_callback */
#define  PANEL_LANG                       23      /* control type: pictRing, callback function: on_lang_change_callback */
#define  PANEL_PATTERNSWITCH              24      /* control type: binary, callback function: on_change_callback */
#define  PANEL_TIMER                      25      /* control type: timer, callback function: draw_to_canvas */
#define  PANEL_RESETTEXTMSG               26      /* control type: textMsg, callback function: (none) */
#define  PANEL_RESETTITLEMSG              27      /* control type: textMsg, callback function: (none) */
#define  PANEL_SAVETOMLMSG                28      /* control type: textMsg, callback function: (none) */
#define  PANEL_LOADTOMLMSG                29      /* control type: textMsg, callback function: (none) */
#define  PANEL_EXPORTPNGMSG               30      /* control type: textMsg, callback function: (none) */
#define  PANEL_AD                         31      /* control type: textMsg, callback function: (none) */

#define  PANEL_2                          2       /* callback function: quality_numbers_panel_callback */
#define  PANEL_2_TABLE                    2       /* control type: table, callback function: (none) */
#define  PANEL_2_TABLE_HEADERS            3       /* control type: table, callback function: (none) */

#define  PANELABOUT                       3       /* callback function: close_panel_about */
#define  PANELABOUT_CLOSEABOUT            2       /* control type: command, callback function: close_about */
#define  PANELABOUT_PICTURE               3       /* control type: picture, callback function: (none) */
#define  PANELABOUT_ABOUTMSG              4       /* control type: textMsg, callback function: (none) */


     /* Control Arrays: */

          /* (no control arrays in the resource file) */


     /* Menu Bars, Menus, and Menu Items: */

#define  MENUBAR                          1
#define  MENUBAR_FILE                     2
#define  MENUBAR_FILE_OPEN                3       /* callback function: menu_callback */
#define  MENUBAR_FILE_SAVE                4       /* callback function: menu_callback */
#define  MENUBAR_FILE_SEPARATOR           5
#define  MENUBAR_FILE_EXPORT              6       /* callback function: menu_callback */
#define  MENUBAR_FILE_QIUT                7
#define  MENUBAR_FILE_PRINT               8       /* callback function: menu_callback */
#define  MENUBAR_FILE_SEPARATOR_3         9
#define  MENUBAR_FILE_QUIT                10      /* callback function: menu_callback */
#define  MENUBAR_EDIT                     11
#define  MENUBAR_EDIT_COPY                12      /* callback function: menu_callback */
#define  MENUBAR_EDIT_SEPARATOR_4         13
#define  MENUBAR_EDIT_RESET               14      /* callback function: menu_callback */
#define  MENUBAR_VIEW                     15
#define  MENUBAR_VIEW_THEME               16      /* callback function: menu_theme_callback */
#define  MENUBAR_VIEW_THEME_SUBMENU       17
#define  MENUBAR_VIEW_THEME_LIGHT         18      /* callback function: menu_theme_callback */
#define  MENUBAR_VIEW_THEME_DARK          19      /* callback function: menu_theme_callback */
#define  MENUBAR_VIEW_THEME_LIGHT_AXIS    20      /* callback function: menu_theme_callback */
#define  MENUBAR_VIEW_THEME_DARK_AXIS     21      /* callback function: menu_theme_callback */
#define  MENUBAR_VIEW_THEME_PRIDE         22      /* callback function: menu_theme_callback */
#define  MENUBAR_VIEW_LANGUAGE            23
#define  MENUBAR_VIEW_LANGUAGE_SUBMENU    24
#define  MENUBAR_VIEW_LANGUAGE_ENGLISH    25      /* callback function: language_callback */
#define  MENUBAR_VIEW_LANGUAGE_GERMAN     26      /* callback function: language_callback */
#define  MENUBAR_HELP                     27
#define  MENUBAR_HELP_HELP                28      /* callback function: menu_help_callback */
#define  MENUBAR_HELP_SEPARATOR_5         29
#define  MENUBAR_HELP_QUALITY             30      /* callback function: quiality_menu_callback */
#define  MENUBAR_HELP_SEPARATOR_2         31
#define  MENUBAR_HELP_ABOUT               32      /* callback function: menu_callback */


     /* Callback Prototypes: */

int  CVICALLBACK close_about(int panel, int control, int event, void *callbackData, int eventData1, int eventData2);
int  CVICALLBACK close_panel_about(int panel, int event, void *callbackData, int eventData1, int eventData2);
int  CVICALLBACK draw_to_canvas(int panel, int control, int event, void *callbackData, int eventData1, int eventData2);
int  CVICALLBACK export_callback(int panel, int control, int event, void *callbackData, int eventData1, int eventData2);
void CVICALLBACK language_callback(int menubar, int menuItem, void *callbackData, int panel);
int  CVICALLBACK listbox_callback(int panel, int control, int event, void *callbackData, int eventData1, int eventData2);
int  CVICALLBACK load_callback(int panel, int control, int event, void *callbackData, int eventData1, int eventData2);
int  CVICALLBACK main_panel_callback(int panel, int event, void *callbackData, int eventData1, int eventData2);
void CVICALLBACK menu_callback(int menubar, int menuItem, void *callbackData, int panel);
void CVICALLBACK menu_help_callback(int menubar, int menuItem, void *callbackData, int panel);
void CVICALLBACK menu_theme_callback(int menubar, int menuItem, void *callbackData, int panel);
int  CVICALLBACK on_change_callback(int panel, int control, int event, void *callbackData, int eventData1, int eventData2);
int  CVICALLBACK on_lang_change_callback(int panel, int control, int event, void *callbackData, int eventData1, int eventData2);
int  CVICALLBACK quality_numbers_panel_callback(int panel, int event, void *callbackData, int eventData1, int eventData2);
void CVICALLBACK quiality_menu_callback(int menubar, int menuItem, void *callbackData, int panel);
int  CVICALLBACK quit_callback(int panel, int control, int event, void *callbackData, int eventData1, int eventData2);
int  CVICALLBACK save_as_callback(int panel, int control, int event, void *callbackData, int eventData1, int eventData2);
int  CVICALLBACK scratch_pad(int panel, int control, int event, void *callbackData, int eventData1, int eventData2);


#ifdef __cplusplus
    }
#endif