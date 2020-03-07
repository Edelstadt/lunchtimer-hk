import 'dart:math';

import 'package:flutter/material.dart';

class BaseNavigator extends StatelessWidget {
  final Widget child;
  BaseNavigator({@required this.child});

  @override
  Widget build(BuildContext context) {
    return Scaffold(
        appBar: AppBar(
          bottom: PreferredSize(
            preferredSize: Size.fromHeight(5),
            child: Flex(
              direction: Axis.horizontal,
              mainAxisAlignment: MainAxisAlignment.center,
              children: <Widget>[
                _NavButton(
                  title: 'Dle restaurací',
                  route: '/',
                ),
                _NavButton(
                  title: 'Dle jídel',
                  route: '/all',
                ),
              ],
            ),
          ),
        ),
        body: child,
    );
  }
}

class _NavButton extends StatelessWidget {
  final String title;
  final String route;

  _NavButton({
    @required this.title,
    @required this.route,
  });

  @override
  Widget build(BuildContext context) {
    return Container(
      width: min(MediaQuery.of(context).size.width / 2, 300),
      height: 60,
      child: FlatButton(
        onPressed: () {
          Navigator.pushReplacementNamed(
            context,
            route
          );
        },
        child: Text(
          title,
          style: Theme.of(context).textTheme.headline1,
        ),
      ),
    );
  }
}
