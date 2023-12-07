namespace Day05;

public class Solver
{
    private List<double> _seeds;
    private List<Mapper> _mappers;

    public Solver()
    {
        GetInputs();
    }

    public double Solve1()
    {
        var locations = new List<double>();
        
        foreach (var seed in _seeds)
        {
            var newValue = seed;
            // Console.Write($"Seed {seed} -   ");

            foreach (var mapper in _mappers)
            {
                newValue = mapper.Map(newValue);
            }
            locations.Add(newValue);
            // Console.WriteLine();
        }
        Console.WriteLine();

        foreach (var location in locations)
        {
            Console.WriteLine($"Location - {location}");
        }

        return locations.Min();
    }
    
    public int Solve2()
    {
        return 1;
    }

    void GetInputs()
    {
        var text = File.ReadAllText("input.txt");
        var sections = text.Split("\r\n\r\n");
        
        _seeds = sections[0].Split(' ').Skip(1).Select(double.Parse).ToList();
        _mappers = new List<Mapper>
        {
            new SoilMapper(sections[1]),
            new FertilizerMapper(sections[2]),
            new WaterMapper(sections[3]),
            new LightMapper(sections[4]),
            new TemperatureMapper(sections[5]),
            new HumidityMapper(sections[6]),
            new LocationMapper(sections[7]),
        };
    }
}

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

public abstract class Mapper
{
    public Mapper(string section)
    {
        var lines = section.Split(Environment.NewLine);
        Ranges = lines.Skip(1).Select(line => new Range(line)).ToList();
    }
    
    public abstract string GetName();
    public abstract int GetOrder();
    public List<Range> Ranges { get; set; }

    public virtual double Map(double x)
    {
        var mapper = Ranges.FirstOrDefault(r => r.IsMatch(x));

        if (mapper == null)
        {
            // Console.Write($" -> {GetName()} {x} (fallback)");
            return x;
        }

        var mapped = mapper.Map(x);
        // Console.Write($" -> {GetName()} {mapped}");
        return mapped;
    }
}

public class SoilMapper : Mapper
{
    public SoilMapper(string line) : base(line) { }
    
    public override string GetName() => "soil";
    public override int GetOrder() => 1;
}

public class FertilizerMapper : Mapper
{
    public FertilizerMapper(string line) : base(line) { }
    
    public override string GetName() => "fertilizer";
    public override int GetOrder() => 2;
}

public class WaterMapper : Mapper
{
    public WaterMapper(string line) : base(line) { }
    
    public override string GetName() => "water";
    public override int GetOrder() => 3;
}

public class LightMapper : Mapper
{
    public LightMapper(string line) : base(line) { }
    
    public override string GetName() => "light";
    public override int GetOrder() => 4;
}

public class TemperatureMapper : Mapper
{
    public TemperatureMapper(string line) : base(line) { }
    
    public override string GetName() => "temperature";
    public override int GetOrder() => 5;
}

public class HumidityMapper : Mapper
{
    public HumidityMapper(string line) : base(line) { }
    
    public override string GetName() => "humidity";
    public override int GetOrder() => 6;
}

public class LocationMapper : Mapper
{
    public LocationMapper(string line) : base(line) { }
    
    public override string GetName() => "location";
    public override int GetOrder() => 7;
}
