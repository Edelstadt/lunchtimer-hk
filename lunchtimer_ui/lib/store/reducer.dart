import 'package:lunchtimer_ui/store/action.dart';
import 'package:lunchtimer_ui/store/state.dart';

AppState appStateReducer(AppState state, action) {
  if (action is StoreMenusAction) {
    return AppState(
      menus: action.menus,
    );
  }

  return state;
}
