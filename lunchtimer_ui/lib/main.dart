import 'package:flutter/material.dart';
import 'package:flutter/widgets.dart';
import 'package:lunchtimer_ui/tab_view.dart';

void main() => runApp(MaterialApp(
  title: 'Lunchtimer',
  themeMode: ThemeMode.dark,
  darkTheme: ThemeData(
    brightness: Brightness.dark,
    textTheme: TextTheme(
      bodyText1: TextStyle(
        fontSize: 20.0,
      ),
      bodyText2: TextStyle(
        fontSize: 20.0,
      ),
      headline1: TextStyle(
        fontSize: 28.0,
        fontWeight: FontWeight.w400,
      ),
      headline2: TextStyle(
        fontSize: 26.0,
        fontWeight: FontWeight.w400,
      ),
    ),
  ),
  home: DefaultTabController(
    initialIndex: 0,
    length: 2,
    child: Scaffold(
      appBar: PreferredSize(
        preferredSize: Size.fromHeight(60),
        child: Center(
          child: Container(
            height: 60,
            constraints: BoxConstraints(
              maxWidth: 1000,
              minWidth: 200,
            ),
            child: TabBar(
              tabs: <Widget>[
                TabBtn('Dle restaurací'),
                TabBtn('Dle jídel'),
              ],
            ),
          ),
        ),
      ),
      body: LunchtimerTabView(),
    ),
  ),
));

class TabBtn extends StatelessWidget {
  String text;

  TabBtn(text) {
    this.text = text;
  }

  @override
  Widget build(BuildContext context) {
    return Container(
      height: 60,
      child: Center(
        child: Text(
            this.text,
            style: Theme.of(context).textTheme.headline1,
        ),
      ),
    );
  }
}
