namespace Day05;

public class Range
{
    public Range(string line)
    {
        var tokens = line.Split(' ');
        
        DestinationStart = double.Parse(tokens[0]);
        SourceStart = double.Parse(tokens[1]);
        Length = double.Parse(tokens[2]);

        SourceEnd = SourceStart + Length - 1;
        DestinationEnd = DestinationStart + Length - 1;
        RangeDelta = DestinationStart - SourceStart;
    }
    
    public double SourceStart { get; set; }
    public double SourceEnd { get; set; }
    public double DestinationStart { get; set; }
    public double DestinationEnd { get; set; }
    public double RangeDelta { get; set; }
    public double Length { get; set; }

    public bool IsMatch(double x)
    {
        return x >= SourceStart && x <= SourceEnd;
    }

    public double Map(double x)
    {
        return x + RangeDelta;
    }
}