import 'package:flutter/foundation.dart';

class AppState {
  final List<Menu> menus;

  AppState({
    @required this.menus,
  });

  AppState.initialState() : menus = List.unmodifiable(<Menu>[]);
}

class Menu {
  final String title;
  final List<Food> dishes;
  final List<Food> soups;

  Menu({
    @required this.title,
    @required this.dishes,
    @required this.soups,
  });

  Menu copyWith({String title, List<Food> dishes, List<Food> soups}) {
    return Menu(
      title: title ?? this.title,
      dishes: dishes ?? this.dishes,
      soups: soups ?? this.soups,
    );
  }
}

class Food {
  final String label;
  final String price;
  final bool marker;

  Food({
    @required this.label,
    @required this.price,
    @required this.marker,
  });

  Food copyWith({String label, String price, bool marker}) {
    return Food(
      label: label ?? this.label,
      price: price ?? this.price,
      marker: marker ?? this.marker,
    );
  }
}
