
public class JedliksToyCar {

    private long dist = 0;
    private float batteryPercent = 100;

    public static JedliksToyCar buy() {
        // throw new UnsupportedOperationException("Please implement the (static) JedliksToyCar.buy()  method");
        return new JedliksToyCar();
    }

    public String distanceDisplay() {
        return "Driven " + dist + " meters";
    }

    public String batteryDisplay() {
        if (batteryPercent == 0) {
            return "Battery empty";
        }
        return "Battery at " + (int) batteryPercent + "%";
    }

    public void drive() {
        if (batteryPercent == 0) {
            return;
        }
        dist += 20;
        batteryPercent -= 1;
    }
}
