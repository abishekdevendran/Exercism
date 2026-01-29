
class BirdWatcher {

    private final int[] birdsPerDay;

    public BirdWatcher(int[] birdsPerDay) {
        this.birdsPerDay = birdsPerDay.clone();
    }

    public int[] getLastWeek() {
        return birdsPerDay;
    }

    public int getToday() {
        return birdsPerDay[6];
    }

    public void incrementTodaysCount() {
        birdsPerDay[6] += 1;
    }

    public boolean hasDayWithoutBirds() {
        for (int el : birdsPerDay) {
            if (el == 0) {
                return true;
            }
        }
        return false;
    }

    public int getCountForFirstDays(int numberOfDays) {
        int ans = 0;
        for (int i = 0; i < Math.min(numberOfDays, 7); i++) {
            ans += birdsPerDay[i];
        }
        return ans;
    }

    public int getBusyDays() {
        int ans = 0;
        for (int el : birdsPerDay) {
            if (el >= 5) {
                ans++;
            }
        }
        return ans;
    }
}
