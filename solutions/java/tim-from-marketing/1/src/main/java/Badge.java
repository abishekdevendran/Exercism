
class Badge {

    public String print(Integer id, String name, String department) {
        department = (department == null) ? "OWNER" : department.toUpperCase();
        if (id == null) {
            return String.format("%s - %s", name, department);
        }
        return String.format("[%d] - %s - %s", id, name, department);
    }
}
