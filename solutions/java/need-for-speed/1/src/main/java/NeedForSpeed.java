
class NeedForSpeed {

    final int speed;
    final int batteryDrain;
    private int battery = 100;
    private int distance = 0;

    NeedForSpeed(int speed, int batteryDrain) {
        this.speed = speed;
        this.batteryDrain = batteryDrain;
    }

    public boolean batteryDrained() {
        return battery < batteryDrain;
    }

    public int distanceDriven() {
        return distance;
    }

    public void drive() {
        if (batteryDrained()) {
            return;
        }
        distance += speed * (battery > batteryDrain ? 1 : battery / batteryDrain);
        battery = Math.max(0, battery-batteryDrain);
    }

    public static NeedForSpeed nitro() {
        return new NeedForSpeed(50, 4);
    }
}

class RaceTrack {

    private final int distance;

    RaceTrack(int distance) {
        this.distance = distance;
    }

    public boolean canFinishRace(NeedForSpeed car) {
        return car.speed * Math.floor(100 / car.batteryDrain) >= distance;
    }
}
