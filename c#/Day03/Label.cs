namespace Day03;

public class Label
{
    public int Row { get; set; }
    public int Col { get; set; }
    public int Length { get; set; }
    public int Value { get; set; }
    public List<Point> AdjacentPoints { get; set; }

    public bool IsAdjacentTo(Point point)
    {
        return AdjacentPoints.Contains(point);
    }
}

public record Point(int Row, int Col);

//
// public record Point
// {
//     public int X { get; set; }
//     public int Y { get; set; }
// }