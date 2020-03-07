import 'package:flutter/material.dart';
import 'package:lunchtimer_ui/styles.dart';
import 'package:lunchtimer_ui/views/baseNavigator.dart';
import 'package:lunchtimer_ui/views/restaurantsList.dart';
import 'package:redux/redux.dart';
import 'package:flutter_redux/flutter_redux.dart';
import 'package:lunchtimer_ui/store/reducer.dart';
import 'package:lunchtimer_ui/store/state.dart';

void main() => runApp(ReduxContainer());

class ReduxContainer extends StatelessWidget {
  @override
  Widget build(BuildContext context) {
    final Store<AppState> store = Store<AppState>(
      appStateReducer,
      initialState: AppState.initialState(),
    );

// TODO pokračování: https://youtu.be/Wj216eSBBWs?t=803
    return StoreProvider<AppState>(
      store: store,
      child: MaterialApp(
        title: 'Lunchtimer',
        themeMode: ThemeMode.dark,
        darkTheme: darkTheme(),
        onGenerateRoute: (routeSettings) {
          if (routeSettings.name == '/all') {
            return PageRouteBuilder(
                pageBuilder: (_context, _anim1, _anim2) => BaseNavigator(
                  child: Text('seznam všech...'),
                )
            );
          }

          return PageRouteBuilder(
              pageBuilder: (_context, _anim1, _anim2) => BaseNavigator(
                child: RestaurantsList(),
              )
          );
        },
        initialRoute: '/',
      ),
    );
  }
}
