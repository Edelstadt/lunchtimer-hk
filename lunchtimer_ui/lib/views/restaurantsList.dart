import 'package:flutter/material.dart';
import 'package:flutter_redux/flutter_redux.dart';
import 'package:redux/redux.dart';
import 'package:lunchtimer_ui/store/state.dart';

class RestaurantsList extends StatefulWidget {
  @override
  State<StatefulWidget> createState() {
    return _RestaurantsListState();
  }
}

class _RestaurantsListState extends State<RestaurantsList> {
  @override
  Widget build(BuildContext context) {
    return StoreConnector<AppState, _ViewModel>(
      converter: (Store<AppState> store) => _ViewModel.create(store),
      builder: (BuildContext context, _ViewModel viewModel) => Text('restauracess'),
    );
  }
}

class _ViewModel {
  final List<Menu> menus;

  _ViewModel({
    this.menus,
  });

  factory _ViewModel.create(Store<AppState> store) {
    return _ViewModel(
      menus: store.state.menus,
    );
  }
}
