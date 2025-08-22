# Exercise 1
- Unit-Tests
- Pentesting
- Functiontests

# Exercise 2
Microsoft Sharepoint.

# Exercise 3
```java
static boolean test_calculate_price() {
    boolean test_ok = true;
    
    // Testfall 1: Keine Extras, kein Rabatt
    double price1 = calculatePrice(20000, 1500, 1000, 0, 0);
    double expected1 = 20000 + 1500 + 1000; // 22500
    test_ok &= assertEqual(price1, expected1, "Test 1");
    
    // Testfall 2: 2 Extras, 5% Händlerrabatt
    double price2 = calculatePrice(20000, 1500, 2000, 2, 5);
    double expected2 = (20000 * 0.95) + 1500 + 2000; // 19000 + 1500 + 2000 = 22500
    test_ok &= assertEqual(price2, expected2, "Test 2");
    
    // Testfall 3: 3 Extras, 10% Rabatt auf Extras
    double price3 = calculatePrice(20000, 1500, 3000, 3, 0);
    double expected3 = 20000 + 1500 + (3000 * 0.90); // 20000 + 1500 + 2700 = 24200
    test_ok &= assertEqual(price3, expected3, "Test 3");

    return test_ok;
}
```

