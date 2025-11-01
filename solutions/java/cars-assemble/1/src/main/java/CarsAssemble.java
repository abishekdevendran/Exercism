import java.util.List;

public class CarsAssemble {

    // a Map of success rates for each speed
     private record SpeedTier(int maxSpeed, double successRate) {}

    private static final List<SpeedTier> SUCCESS_RATES = List.of(
        new SpeedTier(0, 0.0),    // Speed 0
        new SpeedTier(4, 1.0),    // Speeds 1-4
        new SpeedTier(8, 0.90),   // Speeds 5-8
        new SpeedTier(9, 0.80),   // Speed 9
        new SpeedTier(10, 0.77)   // Speed 10
    );

    private double successRate(int speed) {
        if (speed < 0) {
            return 0.0;
        }
        int l = 0, r = SUCCESS_RATES.size()-1;
        while (l<=r){
            int mid = l + (r-l)/2;
            if (speed > SUCCESS_RATES.get(mid).maxSpeed) {
                l = mid + 1;
            } else if (speed < SUCCESS_RATES.get(mid).maxSpeed) {
                r = mid - 1;
            } else {
                return SUCCESS_RATES.get(mid).successRate;
            }
        }
        return SUCCESS_RATES.get(l).successRate;
    }

    public double productionRatePerHour(int speed) {
        return 221 * speed * this.successRate(speed);
    }

    public int workingItemsPerMinute(int speed) {
        return (int) Math.floor(this.productionRatePerHour(speed)/60);
    }
}
